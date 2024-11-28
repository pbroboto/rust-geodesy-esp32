use geodesy::prelude::*;

fn print_coordinates(coords: &[Coor3D; 2]) {
    for (i, &coord) in coords.iter().enumerate() {
        log::info!("Point {}, coordinates: {:?}", i+1, coord);
    }
}

fn main() -> Result<(), Box<Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, ESP32 Rust Geodesy!");
    let pipeline = "inv utm zone=47 ellps=WGS84
             | cart ellps=WGS84
             | helmert translation=-204.4798,-837.8940,-294.7765
             | cart inv ellps=evrst30
             | utm zone=47 ellps=evrst30";

    let mut ctx = Minimal::new();
    let op = ctx.op(pipeline)?;
    log::info!("Test forward pipeline of RG: ");
    let ne1 = Coor3D::raw(685063.5075, 1521137.2111, 0.0);
    let ne2 = Coor3D::raw(681899.5402, 1685083.1430, 0.0);
    let coor1 = [ne1, ne2];
    log::info!("\nFrom grid coordinates of WGS84 UTM zone 47: ");
    print_coordinates(&coor1);

    let mut coor2: [Coor3D; 2] = coor1.clone();
    ctx.apply(op, Fwd, &mut coor2)?;
    log::info!("\nTo grid coordinates of Thailand Indian1975 UTM Zone 47:");
    print_coordinates(&coor2);
    log::info!("\n\n=========Round the trip back!==============");
    log::info!("Test inverse pipeline of RG: ");
    log::info!("\nFrom grid coordinates of Thailand Indian1975 UTM Zone 47:");
    let coor3: [Coor3D; 2] = coor2.clone();
    print_coordinates(&coor3);
    let mut coor4: [Coor3D; 2] = coor3.clone();
    ctx.apply(op, Inv, &mut coor4)?;
    log::info!("\nTo grid coordinates of WGS84 UTM zone 47:");
    print_coordinates(&coor4);
    
    Ok(())
}
