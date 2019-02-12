fn m() {
    // single
    let server = Server::new();
    server.start();
    client.connect(server);

    // multi
    // server
    let server = Server::new();
    server.start();
    //client
    let address = input!(address);
    client.connect(address);
}

fn server() {
    loop {
        let data = receive_from_clients();
        calc_damage(data);
        calc_put_block(data);
        culc_new_geometry(data);
        broadcast(data);
    }
}

fn client() {
    loop {
        draw_gui();
        get_input();
    }
}
