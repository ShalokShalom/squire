#include "value.h"
#include "struct.h"
#include "function.h"
#include "shared.h"
#include "string.h"
#include <string.h>

#define IS_STRING sq_value_is_string
#define AS_STRING sq_value_as_string
#define AS_NUMBER sq_value_as_number
#define AS_INSTANCE sq_value_as_instance
#define AS_FUNCTION sq_value_as_function
#define TYPENAME sq_value_typename
#define AS_STR(c) (AS_STRING(c)->ptr)

void sq_value_clone(sq_value value) {
	switch (SQ_VTAG(value)) {
	case SQ_TSTRING:
		sq_string_clone(AS_STRING(value));
		return;
	case SQ_TINSTANCE:
		sq_instance_clone(AS_INSTANCE(value));
		return;
	case SQ_TFUNCTION:
		sq_function_clone(AS_FUNCTION(value));
		return;
	}
}

void sq_value_free(sq_value value) {
	switch (SQ_VTAG(value)) {
	case SQ_TSTRING:
		sq_string_free(AS_STRING(value));
		return;
	case SQ_TINSTANCE:
		sq_instance_free(AS_INSTANCE(value));
		return;
	case SQ_TFUNCTION:
		sq_function_free(AS_FUNCTION(value));
		return;
	}
}

const char *sq_value_typename(sq_value value) {
	switch (SQ_VTAG(value)) {
	case SQ_TBOOLEAN: return "<boolean>";
	case SQ_TNULL: return "<null>";
	case SQ_TNUMBER: return "<number>";
	case SQ_TSTRING: return "<string>";
	case SQ_TINSTANCE: return AS_INSTANCE(value)->kind->name;
	case SQ_TFUNCTION: return "<function>";
	default: die("unknown tag '%d'", (int) SQ_VTAG(value));
	}
}

bool sq_value_not(sq_value lhs) {
	if (!sq_value_is_boolean(lhs))
		die("cannot logically negate '%s'", TYPENAME(lhs));

	return lhs == SQ_FALSE;
}

bool sq_value_eql(sq_value lhs, sq_value rhs) {
	if (lhs == rhs) return true;

	if (!IS_STRING(lhs) || !IS_STRING(rhs))
		return false;

	return !strcmp(AS_STR(lhs), AS_STR(rhs));
}

bool sq_value_lth(sq_value lhs, sq_value rhs) {
	switch (SQ_VTAG(lhs)) {
	case SQ_TNUMBER:
		if (!sq_value_is_number(rhs))
			break;
		return AS_NUMBER(lhs) < AS_NUMBER(rhs);
	case SQ_TSTRING:
		if (!sq_value_is_string(rhs))
			break;
		return strcmp(AS_STR(lhs), AS_STR(rhs)) < 0;
	}

	die("cannot compare '%s' with '%s'", TYPENAME(lhs), TYPENAME(rhs));
}

bool sq_value_gth(sq_value lhs, sq_value rhs) {
	switch (SQ_VTAG(lhs)) {
	case SQ_TNUMBER:
		if (!sq_value_is_number(rhs))
			break;
		return AS_NUMBER(lhs) > AS_NUMBER(rhs);
	case SQ_TSTRING:
		if (!sq_value_is_string(rhs))
			break;
		return strcmp(AS_STR(lhs), AS_STR(rhs)) > 0;
	}

	die("cannot compare '%s' with '%s'", TYPENAME(lhs), TYPENAME(rhs));
}

sq_value sq_value_add(sq_value lhs, sq_value rhs) {
	switch (SQ_VTAG(lhs)) {
	case SQ_TNUMBER:
		if (!sq_value_is_number(rhs))
			break;
		return sq_value_new_number(AS_NUMBER(lhs) + AS_NUMBER(rhs));

	case SQ_TSTRING:
		if (!sq_value_is_string(rhs))
			break;

		struct sq_string *result = sq_string_alloc(
			AS_STRING(lhs)->length + AS_STRING(rhs)->length
		);

		strcpy(result->ptr, AS_STR(lhs));
		strcat(result->ptr, AS_STR(rhs));
		return sq_value_new_string(result);
	}

	die("cannot add '%s' to '%s'", TYPENAME(lhs), TYPENAME(rhs));
}

sq_value sq_value_sub(sq_value lhs, sq_value rhs) {
	if (!sq_value_is_number(lhs) && !sq_value_is_number(rhs))
		die("cannot subtract '%s' from '%s'", TYPENAME(lhs), TYPENAME(rhs));

	return sq_value_new_number(AS_NUMBER(lhs) - AS_NUMBER(rhs));
}

sq_value sq_value_mul(sq_value lhs, sq_value rhs) {
	switch (SQ_VTAG(lhs)) {
	case SQ_TNUMBER:
		if (!sq_value_is_number(rhs))
			break;
		return sq_value_new_number(AS_NUMBER(lhs) * AS_NUMBER(rhs));

	case SQ_TSTRING:
		if (!sq_value_is_number(rhs))
			break;

		struct sq_string *result = sq_string_alloc(
			AS_STRING(lhs)->length * AS_NUMBER(rhs)
		);
		*result->ptr = '\0';

		for (unsigned i = 0; i < AS_NUMBER(rhs); ++i)
			strcpy(result->ptr, AS_STR(lhs));

		return sq_value_new_string(result);
	}

	die("cannot multiply '%s' to '%s'", TYPENAME(lhs), TYPENAME(rhs));

}

sq_value sq_value_div(sq_value lhs, sq_value rhs) {
	if (!sq_value_is_number(lhs) && !sq_value_is_number(rhs))
		die("cannot divide '%s' from '%s'", TYPENAME(lhs), TYPENAME(rhs));

	if (!AS_NUMBER(rhs)) die("cannot divide by zero");

	return sq_value_new_number(AS_NUMBER(lhs) / AS_NUMBER(rhs));

}

sq_value sq_value_mod(sq_value lhs, sq_value rhs) {
	if (!sq_value_is_number(lhs) && !sq_value_is_number(rhs))
		die("cannot modulo '%s' from '%s'", TYPENAME(lhs), TYPENAME(rhs));

	if (!AS_NUMBER(rhs)) die("cannot modulo by zero");

	return sq_value_new_number(AS_NUMBER(lhs) % AS_NUMBER(rhs));
}