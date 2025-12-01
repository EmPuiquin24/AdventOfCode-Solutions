#include <iostream>
#include <algorithm>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>

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

int SimilarityScore(vector<int> Vector1, vector<int> Vector2) {
	
	int TotalSum = 0;

	for (int i = 0; i < Vector1.size(); i++) {
		int ocurrencies = 0;
		for (int j = 0; j < Vector2.size(); j++) {
			if (Vector1[i] == Vector2[j]) {
				ocurrencies++;
			}
			else if (Vector1[i] < Vector2[j]) {
				break;
			}
		}
		TotalSum += Vector1[i] * ocurrencies;
	}

	return TotalSum;
}



int main() {

	string inputFile = "input.txt";

	// Part 1
	vector<int> Vector1;
	vector<int> Vector2;
	
	GetNumbersToVector(inputFile, Vector1, Vector2);
	SortNumbersInVector(Vector1);
	SortNumbersInVector(Vector2);
	
	int Distance = DistanceBetweenVectors(Vector1, Vector2);

	cout << "The Distance between lists is: " << Distance << endl;
	
	// Part 2
	vector<int> VectorB1;
	vector<int> VectorB2;
	
	GetNumbersToVector(inputFile, VectorB1, VectorB2);
	SortNumbersInVector(VectorB1);
	SortNumbersInVector(VectorB2);

	int ResultB = SimilarityScore(VectorB1, VectorB2);
	cout << "Similarity Score: " << ResultB << endl;

	return 0;
}
