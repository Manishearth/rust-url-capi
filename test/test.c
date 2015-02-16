#include <stdio.h>
#include <string.h>
#include <assert.h>
#include "../src/rust-url-capi.h"

#define TESTURL1 "http://example.com"

int main() {
  // Create URL
  rusturl_ptr url = rusturl_new(TESTURL1, strlen(TESTURL1));
  assert(url); // Check we have a URL

  // Get Host
  rusturl_part part = rusturl_get_host(url);
  char * host = part.content;
    printf("HOST: %s\n", host);
  assert(strcmp(host, "example.com") == 0);
  rusturl_free_part(part);

  // Set Host
  rusturl_set_host(url, "test.com", strlen("test.com"));

  // Test that set_host worked
  part = rusturl_get_host(url);
  host = part.content;
  printf("HOST: %s\n", host);
  assert(strcmp(host, "test.com") == 0);
  rusturl_free_part(part);

  // Free the URL
  rusturl_free(url);

  printf("SUCCESS\n");
  return 0;
}