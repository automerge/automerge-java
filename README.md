# Automerge for Java

This is a java implmentation of automerge. It is implemented by wrapping the
[Rust automerge implementation](https://github.com/automerge/automerge) but you
shouldn't have to think about that. If you _are_ interested take a look at
[HACKING.md](./HACKING.md)

Documentation is mostly just [the API docs](https://www.javadoc.io/doc/org.automerge/automerge).

## Installation

### Maven

```xml
<dependency>
  <groupId>org.automerge</groupId>
  <artifactId>automerge</artifactId>
  <version>0.0.5</version>
</dependency>
```

### Gradle (including android)

```kotlin
dependencies {
    implementation group: 'org.automerge', name: 'automerge', version: "0.0.5"
}
```

### Leiningen

```
  :dependencies [[org.automerge/automerge "0.0.5"]]
```

## A quick example

```java
import org.automerge.ChangeHash;
import org.automerge.Document;
import org.automerge.ObjectId;
import org.automerge.ObjectType;
import org.automerge.Transaction;

public class App {

    public static void main(String[] args) {
        // Create an object
        Document doc = new Document();

        ObjectId text;
        try(Transaction tx = doc.startTransaction()) {
            // Create a text object under the "text" key of the root map
            text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
            tx.spliceText(text, 0, 0, "Hello world");
            tx.commit();
        }

        // save the document
        byte[] docBytes = doc.save();

        // Load the document
        Document doc2 = Document.load(docBytes);
        System.out.println(doc2.text(text).get().toString()); // Prints "Hello world"

        // Modify the doc in doc2
        try(Transaction tx = doc2.startTransaction()) {
            tx.spliceText(text, 5, 0, " beautiful");
            tx.commit();
        }

        // Modify the doc in doc1
        try(Transaction tx = doc.startTransaction()) {
            tx.spliceText(text, 5, 0, " there");
            tx.commit();
        }

        // Merge the changes
        doc.merge(doc2);

        // Prints either "Hello there beautiful world" or "hello beautiful there world"
        // depending on the actor IDs that were generated for each document.
        System.out.println(doc.text(text).get().toString());
    }
}

```
