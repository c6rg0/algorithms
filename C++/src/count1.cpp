#include <iostream>

int main()
{
	int i = 0, n = 0;

	// Get number from the keyboard and initialize i.
    std::cout << "Enter a number and press ENTER: ";
    std::cin >> n;
	i = 1;

	while (i <= n)
    { 
        // while i less than or equal to n,
        std::cout << i << " "; // print i,
		i = i + 1;	  // add 1 to i.
	}

	return 0;
}
