if [ ! -d "./bin" ]
then
  mkdir ./bin
fi
cd bin

g++ ../src/main.cpp -o shell2bin