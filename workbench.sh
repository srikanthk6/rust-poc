#To install and run wrk benchmark tool
sudo apt update
sudo apt-get update
sudo apt install zip -y
sudo apt-get install build-essential libssl-dev git -y

git clone https://github.com/wg/wrk.git
cd wrk

sudo make

sudo cp wrk /usr/local/bin

#Perf test: Replace the ip with the host address
sudo wrk -t100 -c5000 d1m http://<ip>:8080/cpu-intense-task

#Perf test: Replace the ip with the host address
sudo wrk -t100 -c5000 d1m http://<ip>:8080/high-io-task