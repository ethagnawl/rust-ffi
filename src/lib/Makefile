.PHONY: all
all:
	$(MAKE) clean
	$(MAKE) libgreet.a

.PHONY: clean
clean:
		rm -f *.o *.a

.o: greet.c
		gcc -Wall -c *.c

libgreet.a: greet.o
		ar -cvq libgreet.a *.o
