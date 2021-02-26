#include <stdio.h>
#include <time.h>
#include <string.h>

int main() {

	int i;
	int char_written;

	// Create a mulidimensional array to hold the word cosmos. Each index in the first dimention will hold another array of characters comprising the second dimension.

	char *cosmos[6] = {"   o---o  \n  /       \n o        \n  \\       \n   o---o  ", "ooo", "ssss", "mmmm", "ooo", "ss"};

	printf("%s\n", cosmos[0]);

	return 0;
}