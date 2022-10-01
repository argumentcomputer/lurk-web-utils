//mod error;
//mod ipfs;
use ipfs::error::Error;
use ipfs::{dag_get, dag_put};

use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use blstrs::Scalar;
use libipld::serde::{from_ipld, to_ipld};
use lurk::field::LurkField;
use lurk::scalar_store::ScalarStore;
use lurk::store::{Ptr, Store};
use serde::Serialize;

use clap::{AppSettings, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Stores Lurk data on IPFS
  Put(Put),

  /// Retrieves Lurk data from IPFS
  Get(Get),
}

#[derive(Args, Debug)]
struct Put {
  ///Input Lurk data
  #[clap(long, parse(from_os_str))]
  data: PathBuf,

  ///IPFS host
  #[clap(long, default_value = "localhost:5001")]
  host: String,
}

#[derive(Args, Debug)]
struct Get {
  ///Input Lurk data
  #[clap(long)]
  cid: String,

  ///IPFS host
  #[clap(long, default_value = "localhost:5001")]
  host: String,
}

impl Put {
  async fn put(&self) -> Result<(), Error> {
    let store = &mut Store::<Scalar>::default();

    let src = read_from_path(store, &self.data)?;
    store.hydrate_scalar_cache();
    let (scalar_store, _) = ScalarStore::new_with_expr(store, &src);
    let ipld = to_ipld(scalar_store.clone()).unwrap();
    let cid = dag_put(&self.host, ipld)
      .await
      .expect("Failed to store on IPFS");
    println!("{:?}\nstored on IPFS", cid);
    Ok(())
  }
}

impl Get {
  async fn get(&self) -> Result<(), Error> {
    let ipld = dag_get(&self.host, &self.cid)
      .await
      .expect("Failed to retrieve from IPFS");
    let data: ScalarStore<Scalar> = from_ipld(ipld).expect("Invalid Lurk IPLD");
    let mut file = File::create("ipfs_output.txt")?;
    file.write_all(format!("{:?}", data).as_bytes())?;
    println!("Lurk data retrieved from IPFS");
    Ok(())
  }
}

fn read_from_path<P: AsRef<Path>, F: LurkField + Serialize>(
  store: &mut Store<F>,
  path: P,
) -> Result<Ptr<F>, Error> {
  let path = env::current_dir()?.join(path);
  let input = read_to_string(path)?;
  let src = store.read(&input).unwrap();

  Ok(src)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let cli = Cli::parse();

  match &cli.command {
    Command::Put(p) => p.put().await,
    Command::Get(g) => g.get().await,
  }
}
