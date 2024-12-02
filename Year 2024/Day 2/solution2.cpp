#include <iostream>
#include <sstream>
#include <fstream>
#include <string>
#include <vector>

using namespace std;

bool isSafeReport(vector<int> Vector) {
	bool isDecreasing = true;
	bool isIncreasing  = true;

	for(int i = 0; i < Vector.size() - 1; i++) {
		int levelDifference = Vector[i] - Vector[i+1];
			
		if (abs(levelDifference) < 1 || abs(levelDifference) > 3) {
			return false;
		}

		if (levelDifference < 0) {
			isDecreasing = false;
		}
		else {
			isIncreasing = false;
		}

		if (!isDecreasing && !isIncreasing) {	
			return false;
		}
	}
	return true;
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
}

int main() {
	string Filename = "input.txt";
	int nSafeReports = 0;
	
	countReports(Filename, nSafeReports);
	
	cout << "The amount of safe reports are: " << nSafeReports << endl;
	return 0;
}
