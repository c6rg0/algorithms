// Binary search implementation

#include <stdio.h>
#include <string.h>

void binary_search(int list[], int target, int last) {
	int found = 0;
	int index = -1;
	int first = 0;

	while (first <= last && found == 0) {
		int midpoint = (first + last) / 2;

		if (list[midpoint] == target) {
			found = 1;
			index = midpoint;
			printf("Found at index %d\n", index);
		}
		
		else {
			if (list[midpoint] < target) {
				first = midpoint + 1;
			}
			else {
				last = midpoint - 1;
			}
		}
	}
}

int main() {
	int list[] = {1, 4, 81, 90, 103, 153, 241};
	int target = 241;
	int last = (sizeof(list) / sizeof(list[0])) - 1;

	binary_search(list, target, last);
}



