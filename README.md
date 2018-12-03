# holochat-rust

Warning : this code is for debugging purpose only

steps to reproduce the OutofMemory error "error":{"Internal":"{\"kind\":{\"ValidationFailed\":\"Out of memory\"},\"file\":\"core/src/nucleus/ribosome/runtime.rs\",\"line\":\"86\"}"}} 

1. run hc run --port 3400 --package  
2. wait for initialization (creation of a channel and a profile)
3. use the chat or just let it run for approx 8-10mn
4. try to send a message or create a room, watch the hc console to see the message




