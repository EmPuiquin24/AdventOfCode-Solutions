#include <iostream>
#include <fstream>
#include <string>
#include <regex>

using namespace std;

vector<string> PatternMatching(string text, regex pattern) {

	vector<string> coincidences;
	auto wordsBegin = std::sregex_iterator(text.begin(), text.end(), pattern);
	auto wordEnd = std::sregex_iterator();

	for (auto it = wordsBegin; it != wordEnd; it++) {
		coincidences.push_back(it->str());
	}
	return coincidences;
}


int mulMatching(string text, regex pattern) {

	int Summ = 0;

	auto wordsBegin = sregex_iterator(text.begin(), text.end(), pattern);
	auto wordsEnd = sregex_iterator();

	for (auto it = wordsBegin; it != wordsEnd; it++) {
		int number1 = stoi((*it)[1]);
		int number2 = stoi((*it)[2]);

		Summ += number1 * number2;
    }
    return Summ;
}


int main() {
	
	regex firstPattern(R"(^(.*?)don't\(\))");
	regex doPattern(R"(do\(\).*?don't\(\))");
	regex mulPattern(R"(mul\((\d+),(\d+)\))");
	
	// Part One
	string Filename1 = "inputA.txt";
	ifstream inputA(Filename1);
	
	string text1;
	int TotalSum1 = 0;
	
	getline(inputA, text1);
	TotalSum1 += mulMatching(text1, mulPattern);
	
	inputA.close();
	cout << TotalSum1 << endl;


	// Part Two
	string Filename2 = "inputB.txt";
	ifstream AuxiliarInputB(Filename2);
	ifstream inputB(Filename2);
	
	string firstLine;
	string text2;
	int TotalSum2= 0;

	// To get the first active mul before the first do()
	getline(AuxiliarInputB, firstLine);
	vector<string> beforeDoCoincidences = PatternMatching(firstLine, firstPattern);
	int FirstSum = mulMatching(beforeDoCoincidences[0], mulPattern);
	
	TotalSum2 += FirstSum;
	
	AuxiliarInputB.close();
	// To get all the actives mul after the first do()
	getline(inputB, text2);

	int partialSum = 0;
	vector<string> coincidences = PatternMatching(text2, doPattern);
	
	for (int i = 0; i < coincidences.size(); i++) {
		partialSum += mulMatching(coincidences[i], mulPattern);
	}
	TotalSum2 += partialSum;
	
	inputB.close();
	cout << TotalSum2<< endl;

	return 0;
}
