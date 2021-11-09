package io.substrait.util;

import java.util.function.Supplier;

public class Util {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(Util.class);


  public static <T> Supplier<T> memoize(Supplier<T> supplier) {
    return new Memoizer<T>(supplier);
  }

  private static class Memoizer<T> implements Supplier<T> {

    private boolean retrieved;
    private T value;
    private Supplier<T> delegate;

    public Memoizer(Supplier<T> delegate) {
      this.delegate = delegate;
    }

    @Override
    public T get() {
      if (!retrieved) {
        value = delegate.get();
        retrieved = true;
      }
      return value;
    }

  }


  public static class IntRange {
    private final int startInclusive;
    private final int endExclusive;

    public static IntRange of(int startInclusive, int endExclusive) {
      return new IntRange(startInclusive, endExclusive);
    }

    private IntRange(int startInclusive, int endExclusive) {
      this.startInclusive = startInclusive;
      this.endExclusive = endExclusive;
    }

    public int getStartInclusive() {
      return startInclusive;
    }

    public int getEndExclusive() {
      return endExclusive;
    }

    public boolean within(int val) {
      return val >= startInclusive && val < endExclusive;
    }
  }
}
