signature icmp-first-chunk-piece-AABBCCDD { 
  ip-proto == icmp
  payload /.*AABBCCDD.*/
  event "Found AABBCCDD using signature from client"
}

signature icmp6-first-chunk-piece-AABBCCDD { 
  ip-proto == icmp6
  payload /.*AABBCCDD.*/
  event "Found AABBCCDD using signature from client"
}
