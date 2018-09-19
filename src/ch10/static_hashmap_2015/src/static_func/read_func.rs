use static_func::static_kv;
pub fn read_kv() {
    let ref m = static_kv::MAP;
    assert_eq!("foo", *m.get(&0).unwrap_or(&static_kv::NF));
    assert_eq!(static_kv::NF, *m.get(&1).unwrap_or(&static_kv::NF));
}
pub fn rw_mut_kv() -> Result<(), String> {
    {
        let m = static_kv::MAP_MUT.read().map_err(|e| e.to_string())?;
        assert_eq!("bar", *m.get(&0).unwrap_or(&static_kv::NF));
    }
    {
        let mut m = static_kv::MAP_MUT.write().map_err(|e| e.to_string())?;
        m.insert(1, "baz");
    }
    Ok(())
}
