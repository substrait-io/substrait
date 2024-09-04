package io.substrait.examples;

import java.nio.file.Files;
import java.nio.file.Paths;

import io.substrait.plan.Plan;
import io.substrait.plan.ProtoPlanConverter;

public class App {

    public static interface Action {
        public void run(String arg);
    }

    public static void main(String args[]) {
        try {

            if (args.length == 0) {
                args = new String[] { "SparkDataset" };
            }
            String exampleClass = args[0];

            var clz = Class.forName(App.class.getPackageName() + "." + exampleClass);
            var action = (Action) clz.getDeclaredConstructor().newInstance();

            if (args.length == 2) {
                action.run(args[1]);
            } else {
                action.run(null);
            }

        } catch (Exception e) {
            e.printStackTrace();
        }
    }

}
