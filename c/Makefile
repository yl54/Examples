clean:
	rm -f *.o

bit: bit.o
	gcc -Wall -g -std=c11 -o bit bit.o

bit.o: bit.c
	gcc -Wall -g -std=c11 -c bit.c
