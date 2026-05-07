// Linear search implementation

#include <stdio.h>
#include <string.h>

void linear_search(int list[], int target, int len) {
	int index = -1;
	int i = 0;
	int found = 0;

	printf("Declarations complete!\n");
	printf("%d\n", i);

	while (i < len && found == 0) {
		printf("%d\n", i);

		if(list[i] == target) {
			index = i;
			found = 1;
			printf("Found at index %d\n", i);
		}

		i = i + 1;
	}
}

int main() {
	int list[] = {9, 2, 7, 1934, 13, 200, 1};
	int target = 200; // index 5
	int len = sizeof(list) / sizeof(list[0]);
	linear_search(list, target, len);
	return 0;
}

