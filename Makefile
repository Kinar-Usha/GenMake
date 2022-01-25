bin/build: bin/app.o bin/sum.o 
	g++ -std=c++17 bin/app.o bin/sum.o -o bin/build

bin/app.o: source/app.cpp
	g++ --std=c++17 -c source/app.cpp -o bin/app.o

bin/sum.o: includes/sum/sum.cpp includes/sum/sum.h
	g++ --std=c++17 -c includes/sum/sum.cpp -o bin/sum.o

clean:
	rm -rf bin/*.o
