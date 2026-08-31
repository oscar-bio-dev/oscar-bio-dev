#!/bin/bash
set -e

# SetaeSense mTLS Certificate Generation Script
# For Space Grade Hardware Security

mkdir -p certs
cd certs

echo "Generando Root CA (SetaeSense IoT Authority)..."
openssl genrsa -out ca.key 4096
openssl req -x509 -new -nodes -key ca.key -sha256 -days 3650 -out ca.crt \
    -subj "/C=MX/ST=CDMX/L=Mexico City/O=SetaeSense/OU=IoT/CN=SetaeSense Root CA"

echo "Generando certificado para el Servidor (Backend)..."
openssl genrsa -out server.key 2048
openssl req -new -key server.key -out server.csr \
    -subj "/C=MX/ST=CDMX/L=Mexico City/O=SetaeSense/OU=Backend/CN=localhost"
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 365 -sha256

echo "Generando certificado para un cliente IoT (Ejemplo: esp32-node-1)..."
openssl genrsa -out client_esp32_1.key 2048
openssl req -new -key client_esp32_1.key -out client_esp32_1.csr \
    -subj "/C=MX/ST=CDMX/L=Mexico City/O=SetaeSense/OU=EdgeNode/CN=esp32-node-1"
openssl x509 -req -in client_esp32_1.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out client_esp32_1.crt -days 365 -sha256

echo "Certificados generados en ./certs"
chmod 600 *.key
