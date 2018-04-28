package main

import (
    "io"
    "net/http"
)

func hello(w http.ResponseWriter, r *http.Request) {
    io.WriteString(w, "Hello world!")
}

func main() {
    http.HandleFunc("/", hello)

    // This defaults to 0.0.0.0/0, basicaly can listen to anything, binds to all interfaces.
    // There are 2^16 ports available for use.
    // 1000 > port # are reserved for stuff. http = 80, https = 443
    // 1000 < port # can be anything, this is why port 8000 is used in lots of intro examples, b/c it is random
    // port is a place to listen on the machine
    // http.ListenAndServe(":8000", nil)
    http.ListenAndServe("0.0.0.0:8000", nil)
}