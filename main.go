package main

    type Config struct{
    ListenAddress string
}

type Server struct {
    Config
    ln net.Listener

}

func NewServer(cfg Config) *Server {
    if len(cfg.ListenAddr){
        return &Server{
            Config: cfg,
        }
    }
}

func (s *Server) Start() error {
    ln, err := net.Listen("tcp", s.ListenAddress)
}

func main() {

}
