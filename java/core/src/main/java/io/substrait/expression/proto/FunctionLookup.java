package io.substrait.expression.proto;

import io.substrait.function.SimpleExtension;
import io.substrait.proto.Capabilities;
import io.substrait.proto.Plan;
import io.substrait.proto.SimpleExtensionDeclaration;
import io.substrait.proto.SimpleExtensionURI;

import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Maintains a mapping between function anchors and function references. Generates references for new anchors.
 */
public class FunctionLookup {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(FunctionLookup.class);

  private final BidiMap<Integer, SimpleExtension.FunctionAnchor> map = new BidiMap<>();
  private final BidiMap<Integer, String> uriMap = new BidiMap<>();

  private int counter = -1;

  public int getFunctionReference(SimpleExtension.Function declaration) {
    Integer i = map.reverseGet(declaration.getAnchor());
    if (i != null) {
      return i;
    }
    ++counter; // prefix here to make clearer than postfixing at end.
    map.put(counter, declaration.getAnchor());
    return counter;
  }

  public SimpleExtension.ScalarFunctionVariant getScalarFunction(int reference, SimpleExtension.ExtensionCollection extensions) {
    var anchor = map.get(reference);
    if (anchor == null) {
      throw new IllegalArgumentException("Unknown function id. Make sure that the function id provided was shared in the extensions section of the plan.");
    }

    return extensions.getScalarFunction(anchor);
  }

  public SimpleExtension.AggregateFunctionVariant getAggregateFunction(int reference, SimpleExtension.ExtensionCollection extensions) {
    var anchor = map.get(reference);
    if (anchor == null) {
      throw new IllegalArgumentException("Unknown function id. Make sure that the function id provided was shared in the extensions section of the plan.");
    }

    return extensions.getAggregateFunction(anchor);
  }

  public void addFunctionsToPlan(Plan.Builder builder) {
    var uriPos = new AtomicInteger(1);
    var uris = new HashMap<String, SimpleExtensionURI>();

    var extensionList = new ArrayList<SimpleExtensionDeclaration>();
    for(var e : map.forwardMap.entrySet()) {
      SimpleExtensionURI uri = uris.computeIfAbsent(e.getValue().namespace(),
          k -> SimpleExtensionURI.newBuilder().setExtensionUriAnchor(uriPos.getAndIncrement()).setUri(k).build());
      var decl = SimpleExtensionDeclaration.newBuilder().setExtensionFunction(
          SimpleExtensionDeclaration.ExtensionFunction.newBuilder()
              .setFunctionAnchor(e.getKey())
              .setName(e.getValue().key())
              .setExtensionUriReference(uri.getExtensionUriAnchor()))
          .build();
      extensionList.add(decl);
    }

    builder.addAllExtensionUris(uris.values());
    builder.addAllExtensions(extensionList);
  }

  /**
   * We don't depend on guava...
   */
  private static class BidiMap<T1, T2> {

    private final Map<T1, T2> forwardMap = new HashMap<>();
    private final Map<T2, T1> reverseMap = new HashMap<>();

    public T2 get(T1 t1) {
      return forwardMap.get(t1);
    }

    public T1 reverseGet(T2 t2) {
      return reverseMap.get(t2);
    }

    public void put(T1 t1, T2 t2) {
      forwardMap.put(t1, t2);
      reverseMap.put(t2, t1);
    }
  }
}
