#include <math.h>
#include <iostream>
#include <string>
bool isPrime(int num)
{
    if (num <= 1)
    {
        return false;
    }
    for (int n = 2; n < floor(sqrt(num)) + 1; n++)
    {
        if (num % n == 0)
        {
            return false;
        }
    }
    return true;
}
void printArr(int *arrPtr, int arrLen)
{
    for (int i = 0; i < arrLen - 1; i++)
    {
        std::cout << *(arrPtr + i) << "\n";
    }
}
void getPrimes(int *primesArrPtr, int primesArrLen)
{
    int x = 1;
    int i = 0;

    while (i < primesArrLen)
    {
        x++;
        if (isPrime(x))
        {
            *(primesArrPtr + i) = x;
            i++;
        }
    }
}
bool doConcat(int *arrPtr, int arrLen)
{
    for (int i = 0; i < arrLen - 1; i++)
    {
        int index = i + 1;
        while (index > 0)
        {
            if (index != i)
            {
                std::string concattedStr1 = std::to_string(*(arrPtr + i)) + std::to_string(*(arrPtr + index));
                std::string concattedStr2 = std::to_string(*(arrPtr + index)) + std::to_string(*(arrPtr + i));
                if (arrPtr[i] == arrPtr[index])
                {
                    return false;
                }
                if (!isPrime(std::stoi(concattedStr1)) || !isPrime(std::stoi(concattedStr2)))
                {
                    return false;
                }
            }
            index--;
        }
    }
    return true;
}
int main()
{
    const int arrLen = 10000;
    int primesArr[arrLen];
    // int arr[2] = {0,1};
    // std::cout << doConcat(arr, 2);
    getPrimes(primesArr, arrLen);
    for (int i = 0; i < arrLen; i++)
    {
        for (int n = 0; n < arrLen; n++)
        {
            int arr[2] = {primesArr[i], primesArr[n]};
            if (doConcat(arr, 2))
            {
                for (int x = 0; x < arrLen; x++)
                {
                    for (int y = 0; y < arrLen; y++)
                    {
                        int arr[4] = {primesArr[i], primesArr[n], primesArr[x], primesArr[y]};
                        if (doConcat(arr, 4))
                        {
                            for (int z = 0; z < arrLen; z++)
                            {
                                int arr[5] = {primesArr[i],
                                              primesArr[n],
                                              primesArr[x],
                                              primesArr[y],
                                              primesArr[z]};
                                if (doConcat(arr, 5))
                                {
                                    printArr(arr, 5);
                                    // return 0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // printArr(primesArr, arrLen);
    return 0;
}