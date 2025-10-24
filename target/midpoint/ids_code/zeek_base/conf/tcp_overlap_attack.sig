signature tcp-attack-get-client { 
  ip-proto == tcp
  src-ip == 192.168.56.32
  dst-ip == 192.168.56.33
  payload /.*GET \/malware.exe HTTP\/1.1.*/
  event "Found 'GET /malware.exe HTTP 1.1' using signature from client"
}

signature tcp-attack-ssh-client { 
  ip-proto == tcp
  src-ip == 192.168.56.32
  dst-ip == 192.168.56.33
  payload /.*SSH-2.0-fake_0.1.*/
  event "Found 'SSH-2.0-fake_0.1' using signature from client"
}

signature tcp-attack-get-server { 
  ip-proto == tcp
  src-ip == 192.168.56.33
  dst-ip == 192.168.56.32
  payload /.*malware.exe */
  event "Found 'GET /malware.exe HTTP 1.1' using signature from server"
}

signature tcp-attack-ssh-server { 
  ip-proto == tcp
  src-ip == 192.168.56.33
  dst-ip == 192.168.56.32
  payload /.*SSH-2.0.*/
  event "Found 'ssh-2.0-fake-0.1' using signature from server"
}