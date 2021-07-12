#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "program.h"
#include "shared.h"

#define ERROR(msg) (perror(msg),exit(1))

char *read_file(const char *filename) {
	FILE *stream = fopen(filename, "r");
	if (!stream)
		ERROR("unable to open input file");

	if (fseek(stream, 0, SEEK_END))
		ERROR("unable to seek to end");

	long length = ftell(stream);
	if (fseek(stream, 0, SEEK_SET))
		ERROR("unable to seek to start");

	char *contents = xmalloc(length + 1);
	contents[length] = '\0';
	fread(contents, 1, length, stream);

	if (ferror(stream))
		ERROR("unable to read contents: %s");
	if (fclose(stream) == EOF)
		ERROR("unable to close stream: %s");

	return contents;
}

int main(int argc, const char **argv) {
	if (argc < 3 || (strcmp(argv[1], "-e") && strcmp(argv[1], "-f"))) {
		fprintf(stderr, "usage: %s (-e 'expr' | -f 'filename')\n", argv[0]);
		return 1;
	}

	struct sq_program *program;

	if (argv[1][1] == 'e') {
		program = sq_program_compile(argv[2]);
	} else {
		char *contents = read_file(argv[2]);
		program = sq_program_compile(contents);
		free(contents);
	}

	sq_program_run(program, argc - 3, argv + 3);
	sq_program_free(program);

	return 0;
}

