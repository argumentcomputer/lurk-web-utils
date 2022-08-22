pub mod ipfs;

//pub async fn put(&self) -> Result<(), Error> {
//  let store = &mut Store::<Scalar>::default();
//  
//  let src = read_from_path(store, &self.data)?;
//  store.hydrate_scalar_cache();
//  let (scalar_store, _) = ScalarStore::new_with_expr(store, &src);
//  let ipld = to_ipld(scalar_store.clone()).unwrap();
//  let cid = dag_put(&self.host, ipld)
//    .await
//    .expect("Failed to store on IPFS");
//  println!("{:?}\nstored on IPFS", cid);
//  Ok(())
//}
//
//pub async fn get(&self) -> Result<(), Error> {
//  let ipld = dag_get(&self.host, &self.cid)
//    .await
//    .expect("Failed to retrieve from IPFS");
//  let data: ScalarStore<Scalar> = from_ipld(ipld).expect("Invalid Lurk IPLD");
//  let mut file = File::create("ipfs_output.txt")?;
//  file.write_all(format!("{:?}", data).as_bytes())?;
//  println!("Lurk data retrieved from IPFS");
//  Ok(())
//}


#[cfg(test)]
mod test {
  
  //##[tokio::test]
  //#async fn lurk_roundtrip() -> Result<(), reqwest::Error> {
  //#  let mut store_in = Store::<Fr>::default();
  //#  let expr = store_in.read("symbol").unwrap();
  //#  store_in.hydrate_scalar_cache();
  //#  let (scalar_store_in, _) = ScalarStore::new_with_expr(&store_in, &expr);
  //#  
  //#  let ipld = to_ipld(scalar_store_in.clone()).unwrap();
  //#  let cid = dag_put(&String::from("localhost:5001"), ipld).await?;
  //#  
  //#  let ipld_out = dag_get(&String::from("localhost:5001"), &cid).await?;
  //#  let scalar_store_out = from_ipld(ipld_out).unwrap();
  //#  
  //#  assert_eq!(scalar_store_in, scalar_store_out);
  //#  Ok(())
  //#}
}
