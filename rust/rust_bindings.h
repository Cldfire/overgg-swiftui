#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum matchBriefTypeWrapper {
  IN_FUTURE,
  LIVE,
  COMPLETED,
};
typedef uint8_t matchBriefTypeWrapper;

typedef struct downloaderWrapper downloaderWrapper;

typedef struct mainPageScraperWrapper mainPageScraperWrapper;

typedef struct {
  char *name;
  char *series;
} eventInfoWrapper;

typedef struct {
  uint8_t val;
  bool is_present;
} LameOption_u8;

typedef struct {
  char *name;
  LameOption_u8 maps_won;
} teamCompletedMatchBriefInfoWrapper;

typedef struct {
  int64_t val;
  bool is_present;
} LameOption_i64;

typedef struct {
  eventInfoWrapper event;
  teamCompletedMatchBriefInfoWrapper *teams_ptr;
  int teams_len;
  LameOption_i64 scheduled_timestamp;
} matchBriefInfoWrapper;

void downloaderFree(downloaderWrapper *ptr);

int32_t downloaderGetMainPage(const downloaderWrapper *ptr, mainPageScraperWrapper **inout_ptr);

/**
 * Creates a new `Downloader` for you to use opaquely.
 */
void downloaderNew(downloaderWrapper **inout_ptr);

void mainPageScraperFree(mainPageScraperWrapper *ptr);

int32_t mainPageScraperGetMatchesBrief(const mainPageScraperWrapper *ptr,
                                       matchBriefTypeWrapper _type,
                                       matchBriefInfoWrapper **inout_ptr,
                                       int *inout_len);

void matchesBriefInfoVecFree(matchBriefInfoWrapper *ptr, int len);
