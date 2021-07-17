#include <stdio.h>
#include "build/bug_tracker.h"

int main() {
    // get a pointer to a new BugTracker
    struct BugTracker *bt = new_bugtracker();

    // pop the highest severity bug from the max-heap
    struct Bug *crit_bug = get_next_bug(bt);
    printf("critical bug id %s\n", crit_bug->id);
    printf("critical bug severity %d\n", crit_bug->severity);

    // pop the next highest severity bug from the max-heap
    struct Bug *low_bug = get_next_bug(bt);
    printf("low bug id %s\n", low_bug->id);
    printf("low bug severity %d\n", low_bug->severity);

    // free the two bugs
    free_bug(crit_bug);
    free_bug(low_bug);

    // free the bug tracker (still has one bug in the tracker, which will also be freed)
    free_bugtracker(bt);

    // ==249749== HEAP SUMMARY:
    // ==249749==     in use at exit: 0 bytes in 0 blocks
    // ==249749==   total heap usage: 8 allocs, 8 frees, 1,150 bytes allocated
    // ==249749==
    // ==249749== All heap blocks were freed -- no leaks are possible

    return 0;
}