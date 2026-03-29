#include <iostream>
#include <vector>
#include <unordered_set>
// #include <set>
#include <algorithm>
#include <cassert>

using namespace std;


class Solution {
public:
	static bool hash_simple(vector<int> &nums) {
		unordered_set<int> seen;
		for (int a : nums){
			if (!seen.insert(a).second)
				return true;
		}
		return false;
	}
	static bool hash_iter(vector<int> &nums){
		unordered_set<int> seen;
		return any_of(nums.begin(), nums.end(), [&seen](int a){
		return !seen.insert(a).second;
		});
	}
	static bool sort_simple(vector<int> &nums) {
		sort(nums.begin(),nums.end());
		return adjacent_find(nums.begin(),nums.end()) !=nums.end();
	}
};

void test_suite(bool (*f)(vector<int> &), const string& name){
	vector<int> num1 = {1,2,3,1};
	vector<int> num2 = {1,2,3,4};

	assert(f(num1) == true);
	assert(f(num2) == false);

	cout<<name<<" passed"<<endl;
}


int main() {
	test_suite(Solution::hash_simple, "hash_simple");
	test_suite(Solution::hash_iter, "hash_iter");
	test_suite(Solution::sort_simple, "sort_simple");
	return 0;
}
