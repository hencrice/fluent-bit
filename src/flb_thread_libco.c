
void flb_thread_resume_non_inline(struct flb_thread *th) {
    flb_thread_resume(th);
}

void flb_thread_yield_non_inline(struct flb_thread *th, int ended) {
    flb_thread_yield(th, ended);
}
