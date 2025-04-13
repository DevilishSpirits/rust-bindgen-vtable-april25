#include <stdio.h>
struct BugIllustration {
	virtual ~BugIllustration() = default;
	virtual void fn1() = 0;
	virtual void fn2() = 0;
	virtual void fn3() = 0;
};

struct BugIllustrationImpl: public BugIllustration {
	~BugIllustrationImpl() override {};
	void fn1() override {
		puts("fn1()");
	}
	void fn2() override {
		puts("fn2()");
	}
	void fn3() override {
		puts("fn3()");
	}
};

BugIllustration* new_bug_illustration() {
	return new BugIllustrationImpl();
}
void free_bug_illustration(BugIllustration* bug) {
	return delete bug;
}

#ifdef MAKE_MAIN
int main() {
	BugIllustration* bug = new_bug_illustration();
	bug->fn3();
	free_bug_illustration(bug);
	return 0;
}
#endif
