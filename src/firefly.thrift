struct Position {
  1: required i16 x,
  2: required i16 y
}

struct Firefly {
  1: required Position position,
  2: required double phase,
}

service FireflyService {
  double getPhaseByFireflyPosition(1: Position position),
  void sendPhaseUpdate(1: Firefly firefly),
  list<Firefly> getFireflies(),
}
