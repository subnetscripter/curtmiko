#!/usr/bin/bash


IP=$1
PORT=$2


SSH_STRING="ssh -o StrictHostKeyChecking=accept-new -o KexAlgorithms=+diffie-hellman-group1-sha1,diffie-hellman-group-exchange-sha1,diffie-hellman-group14-sha1 -o HostKeyAlgorithms=+ssh-rsa,ssh-dss -o PubkeyAcceptedKeyTypes=+ssh-rsa,ssh-dss -o Ciphers=+aes128-cbc,aes192-cbc,aes256-cbc -p $PORT $IP"

$SSH_STRING



