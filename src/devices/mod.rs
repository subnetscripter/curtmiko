use std::net::IpAddr;

enum ConnectionType{
    SSH,
    Telnet,
    API
}


struct BaseDevice{
    hostname: String,
    management: IpAddr,
    connection: ConnectionType,
}

enum JuniperType{
}

struct Juniper{
    model: JuniperType,
    device: BaseDevice,

}
