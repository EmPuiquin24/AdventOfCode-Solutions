#include <iostream>
#include <algorithm>
#include <string>
#include <vector>
#include <fstream>

using namespace std;

void GetNumbersToVector(string& Filename, vector<int>& Vector1, vector<int>& Vector2) {
	ifstream file(Filename);
	int Column1, Column2;

	while (file >> Column1 >> Column2) {
		Vector1.push_back(Column1);
		Vector2.push_back(Column2);
	}
}

void SortNumbersInVector(vector<int>& Vector) {
	std::sort(Vector.begin(), Vector.end());
}

int DistanceBetweenVectors(vector<int> Vector1, vector<int> Vector2) {

	int Sum = 0;
	for (int i = 0; i < Vector1.size(); i++) {
		Sum += abs(Vector1[i] - Vector2[i]);
	}
	return Sum;
}

int main() {

	string inputFile = "input.txt";
	vector<int> Vector1;
	vector<int> Vector2;

	GetNumbersToVector(inputFile, Vector1, Vector2);
	SortNumbersInVector(Vector1);
	SortNumbersInVector(Vector2);
	
	int Distance = DistanceBetweenVectors(Vector1, Vector2);

	cout << "The Distance between lists is: " << Distance << endl;

	return 0;
}
