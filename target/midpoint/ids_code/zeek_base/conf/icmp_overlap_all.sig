signature test-icmp-AABBCCDD { 
  ip-proto == icmp
  payload /.*AABBCCDD.*/
  event "Found AABBCCDD using signature from client"
}

signature test-icmp-AABBDDCC { 
  ip-proto == icmp
  payload /.*AABBDDCC.*/
  event "Found AABBDDCC using signature from client"
}

signature test-icmp-AACCBBDD { 
  ip-proto == icmp
  payload /.*AACCBBDD.*/
  event "Found AACCBBDD using signature from client"
}

signature test-icmp-AACCDDBB { 
  ip-proto == icmp
  payload /.*AACCDDBB.*/
  event "Found AACCDDBB using signature from client"
}

signature test-icmp-AADDBBCC { 
  ip-proto == icmp
  payload /.*AADDBBCC.*/
  event "Found AADDBBCC using signature from client"
}

signature test-icmp-AADDCCBB { 
  ip-proto == icmp
  payload /.*AADDCCBB.*/
  event "Found AADDCCBB using signature from client"
}



signature test-icmp-BBAACCDD { 
  ip-proto == icmp
  payload /.*BBAACCDD.*/
  event "Found BBAACCDD using signature from client"
}

signature test-icmp-BBAADDCC { 
  ip-proto == icmp
  payload /.*BBAADDCC.*/
  event "Found BBAADDCC using signature from client"
}

signature test-icmp-BBCCAADD { 
  ip-proto == icmp
  payload /.*BBCCAADD.*/
  event "Found BBCCAADD using signature from client"
}

signature test-icmp-BBCCDDAA { 
  ip-proto == icmp
  payload /.*BBCCDDAA.*/
  event "Found BBCCDDAA using signature from client"
}

signature test-icmp-BBDDAACC { 
  ip-proto == icmp
  payload /.*BBDDAACC.*/
  event "Found BBDDAACC using signature from client"
}

signature test-icmp-BBDDCCAA { 
  ip-proto == icmp
  payload /.*BBDDCCAA.*/
  event "Found BBDDCCAA using signature from client"
}
