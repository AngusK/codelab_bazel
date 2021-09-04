package examples.java;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.assertEquals;

import examples.proto.Person;

/**
 * Tests different numbers of arguments to main().
 */
public class PersonProtoTest {
  Person.Builder mBuilder;
  Person mPerson;

  @BeforeEach
  void getBuilder() {
    mBuilder = Person.newBuilder();
    mPerson = mBuilder.setName("hello test").build();
  }

  @Test
  void testPersonProto() {
    assertEquals("hello test", mPerson.getName());
  }

}
