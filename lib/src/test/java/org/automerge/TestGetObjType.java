package org.automerge;

import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestGetObjType {

    @Test
    public void testGetObjType() {
        Document doc = new Document();
        ObjectId map;
        ObjectId list;
        ObjectId text;
        try (Transaction tx = doc.startTransaction()) {
            map = tx.set(ObjectId.ROOT, "map", ObjectType.MAP);
            list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
            text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
            tx.commit();
        }

        // make an object ID from a different document
        Document otherDoc = new Document();
        ObjectId missingObj;
        try (Transaction tx = otherDoc.startTransaction()) {
            missingObj = tx.set(ObjectId.ROOT, "other", ObjectType.MAP);
            tx.commit();
        }

        Assertions.assertEquals(Optional.of(ObjectType.MAP), doc.getObjectType(map));
        Assertions.assertEquals(Optional.of(ObjectType.LIST), doc.getObjectType(list));
        Assertions.assertEquals(Optional.of(ObjectType.TEXT), doc.getObjectType(text));
        Assertions.assertEquals(Optional.empty(), doc.getObjectType(missingObj));

        // now the same tests but in a transaction
        try (Transaction tx = doc.startTransaction()) {
            Assertions.assertEquals(Optional.of(ObjectType.MAP), tx.getObjectType(map));
            Assertions.assertEquals(Optional.of(ObjectType.LIST), tx.getObjectType(list));
            Assertions.assertEquals(Optional.of(ObjectType.TEXT), tx.getObjectType(text));
            Assertions.assertEquals(Optional.empty(), tx.getObjectType(missingObj));
        }
    }

}
