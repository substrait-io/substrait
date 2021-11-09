package io.substrait.type;

public interface TypeVisitor<R, E extends Throwable> {
  R visit(Type.Bool type) throws E;

  R visit(Type.I8 type) throws E;

  R visit(Type.I16 type) throws E;

  R visit(Type.I32 type) throws E;

  R visit(Type.I64 type) throws E;

  R visit(Type.FP32 type) throws E;

  R visit(Type.FP64 type) throws E;

  R visit(Type.Str type) throws E;

  R visit(Type.Binary type) throws E;

  R visit(Type.Date type) throws E;

  R visit(Type.Time type) throws E;

  R visit(Type.TimestampTZ type) throws E;

  R visit(Type.Timestamp type) throws E;

  R visit(Type.IntervalYear type) throws E;

  R visit(Type.IntervalDay type) throws E;

  R visit(Type.UUID type) throws E;

  R visit(Type.FixedChar type) throws E;

  R visit(Type.VarChar type) throws E;

  R visit(Type.FixedBinary type) throws E;

  R visit(Type.Decimal type) throws E;

  R visit(Type.Struct type) throws E;

  R visit(Type.ListType type) throws E;

  R visit(Type.Map type) throws E;


  public static abstract class TypeThrowsVisitor<R, E extends Throwable> implements TypeVisitor<R, E> {

    private final String unsupportedMessage;

    protected TypeThrowsVisitor(String unsupportedMessage) {
      this.unsupportedMessage = unsupportedMessage;
    }

    protected final UnsupportedOperationException t() {
      throw new UnsupportedOperationException(unsupportedMessage);
    }
    
    @Override
    public R visit(Type.Bool type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.I8 type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.I16 type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.I32 type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.I64 type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.FP32 type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.FP64 type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Str type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Binary type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Date type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Time type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.TimestampTZ type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Timestamp type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.IntervalYear type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.IntervalDay type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.UUID type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.FixedChar type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.VarChar type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.FixedBinary type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Decimal type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Struct type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.ListType type) throws E {
      throw t();
    }

    @Override
    public R visit(Type.Map type) throws E {
      throw t();
    }
  }
}
