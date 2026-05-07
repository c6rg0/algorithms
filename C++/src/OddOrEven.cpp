#include <iostream>

int main()
{
	int n = 0, remainder = 0;
	
    std::cout << "Enter a number and press ENTER: ";
    std::cin >> n;
	remainder = n % 2;
	
	if (remainder == 0)
    {
        std::cout << "The number is even." << endl;
	} 
    else 
    {
        std::cout << "The number is odd" << endl;
	}

	return 0;
}

