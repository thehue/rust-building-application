fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run() // 리턴하지 않고 TCP 소켓에 생긴 새로운 연결을 기다리며 영원히 반복
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    // self - 구조체의 인스턴스
    // run() 함수를 나가게 되면 구조체는 실제로 할당이 해제됨
    //  함수를 마칠 때 구조체를 할당 해제하고 싶지 않다면 &self로 지정 - 값 변경시에는 &mut self
    fn run(self) {}
}
