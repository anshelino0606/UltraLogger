#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

constexpr static const uint64_t LOGGING_THREAD_TIMEOUT = 1;

constexpr static const uintptr_t NUM_LOGGING_THREADS = 4;

constexpr static const uintptr_t NO_INIT_STATE = 0;

constexpr static const uintptr_t DO_INIT_STATE = 1;

constexpr static const uintptr_t INIT_STATE = 2;

enum class LevelFilter : uintptr_t {
  Off,
  Prod,
  Debug,
  Trace,
};

enum class LogLevel {
  None = 0,
  Prod = 1,
  Debug = 2,
  Trace = 3,
};

struct AtomicUsize;

struct LogData {
  const char *args;
  LogLevel level;
  const char *source;
};

struct FfiStringResult {
  const char *result;
  const char *error;
};

extern const AtomicUsize STATE;

extern const AtomicUsize MAX_LOG_LEVEL_FILTER;

void start_logging();

void log_message(LogData log_data);

void log_trace(const char *source, const char *message);

void log_debug(const char *source, const char *message);

void log_prod(const char *source, const char *message);

FfiStringResult read_logs(const char *file_path);

void set_log_level(LogLevel log_level);

void flush_logger();

void cleanup_logger();

} // extern "C"
