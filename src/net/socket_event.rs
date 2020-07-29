use rua_value_list::VarList;

pub struct  SocketEvent {
    socket_fd: i32,
    cookie: u32,
    client_ip: String,
    server_port: u16,
    var_list: VarList,
    out_cache: VarList,
    online: bool,
    websocket: bool,
}

impl SocketEvent {
    pub fn new(socket_fd: i32, client_ip: String, server_port: u16) -> SocketEvent {
        SocketEvent {
            socket_fd,
            cookie: 0,
            client_ip,
            server_port,
            var_list: VarList::new(),
            out_cache: VarList::new(),
            online: true,
            websocket: false,
        }
    }

    pub fn get_socket_fd(&self) -> i32 {
        self.socket_fd
    }

    pub fn get_client_ip(&self) -> String {
        self.client_ip.clone()
    }

    pub fn get_server_port(&self) -> u16 {
        self.server_port
    }

    pub fn get_cookie(&self) -> u32 {
        self.cookie
    }

    pub fn set_cookie(&mut self, cookie: u32) {
        self.cookie = cookie;
    }

    pub fn get_var_list(&mut self) -> &mut VarList {
        &mut self.var_list
    }

    pub fn get_out_cache(&mut self) -> &mut VarList {
        &mut self.out_cache
    }

    pub fn set_online(&mut self, online: bool) {
        self.online = online;
    }

    pub fn get_online(&self) -> bool {
        self.online
    }

    pub fn set_websocket(&mut self, websocket: bool) {
        self.websocket = websocket;
    }

    pub fn is_websocket(&self) -> bool {
        self.websocket
    }
 }