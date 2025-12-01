#include <iostream>
#include <sstream>
#include <fstream>
#include <string>
#include <vector>

using namespace std;

bool isSafeReport(vector<int> Vector) {
	bool isIncreasing = (Vector[1] - Vector[0] > 0);
	bool isDecreasing = (Vector[1] - Vector[0] < 0);

	for(int i = 0; i < Vector.size() - 1; i++) {
		int levelDifference = Vector[i+1] - Vector[i];
			
		if (abs(levelDifference) < 1 || abs(levelDifference) > 3) {
			return false;
		}
		if (levelDifference < 0 &&  isIncreasing) {
			return false;	
		}
		if (levelDifference > 0 && isDecreasing)
			return false;	
		}
	return true;
}

bool isDampenerReport(vector<int> Vector) {
	// Fuerza Bruta aaaaaah algorithm
	if (isSafeReport(Vector)) {
		return true;
	}

	for (int i = 0; i < Vector.size() ; i++) {
		vector<int> Vector2 = Vector;
		Vector2.erase(Vector2.begin() + i);

		if (isSafeReport(Vector2)) {
			return true;
		}
	}
	return false;
}

void countReports(string Filename, int& nSafeReports) {

	ifstream file(Filename);
	string row;

	while (getline(file, row)) {
		int number;
		vector<int> Vector;
		stringstream ss(row);
		
		while(ss >> number) {
			Vector.push_back(number);
		}

		if (isSafeReport(Vector)) {
			nSafeReports++;	
		}
	}

	file.close();
}


void countDampenerReports(string Filename, int& nSafeReports) {
	
	ifstream file2(Filename);
	string row;

	while (getline(file2, row)) {
		int number;
		vector<int> Vector;
		stringstream ss(row);
		
		while(ss >> number) {
			Vector.push_back(number);
		}

		if (isDampenerReport(Vector)) {
			nSafeReports++;	
		}
	}
	
	file2.close();
}

int main() {
	string Filename = "input.txt";

	// Part 1
	int nSafeReports = 0;
	
	countReports(Filename, nSafeReports);

	cout << "The amount of safe reports are: " << nSafeReports << endl;
	
	// Part 2
	int nDampenerReports= 0;

	countDampenerReports(Filename, nDampenerReports);

	cout << nDampenerReports << endl;

	return 0;
}
