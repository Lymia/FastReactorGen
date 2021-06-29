pub mod model;
mod serialization;

pub fn test() {
    let data = include_bytes!("../../../nc-reactor-generator/src/configurations/nuclearcraft.ncpf");
    let (value, data) = serialization::ncpf::simplelibrary::decode(data).unwrap();
    println!("{}", value);
    let (value, data) = serialization::ncpf::simplelibrary::decode(data).unwrap();
    println!("{}", value);

    println!("{:#?}", serde_json::from_value::<serialization::ncpf::ncpf11::NCPF11>(value).unwrap());
}
