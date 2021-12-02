use crate::lsystem::{Instruction, Instructions};

pub trait Graphics<R> {
    fn draw_line(&mut self, c_0: (f32, f32), c_1: (f32, f32)) -> Result<(), R>;
}

#[derive(Clone)]
pub struct Turtle<'a, 'b, 'c, 'd, 'e> {
    x: f32,
    y: f32,
    angle: f32,
    config: &'a TurtleConfig<'b, 'c, 'd, 'e>,
}

pub struct TurtleConfig<'a, 'b, 'c, 'd> {
    delta_ang: f32,
    stepsize: f32,
    draw_forward: &'a str,
    draw_backward: &'b str,
    forward: &'c str,
    backwards: &'d str,
}

impl<'a, 'b, 'c, 'd> Default for TurtleConfig<'a, 'b, 'c, 'd> {
    fn default() -> Self {
        Self::new()
    }
}

enum Step {
    Forward,
    Backward,
    DrawForward,
    DrawBackward,
}

impl<'a, 'b, 'c, 'd> TurtleConfig<'a, 'b, 'c, 'd> {
    pub fn new() -> Self {
        Self {
            delta_ang: std::f32::consts::PI / 4.0,
            stepsize: 1.0,
            draw_forward: "F",
            draw_backward: "f",
            forward: "",
            backwards: "",
        }
    }

    pub fn create_turtle<'e>(&'e self) -> Turtle<'e, 'a, 'b, 'c, 'd> {
        Turtle::with_config(self)
    }

    pub fn delta_ang(self, delta_ang: f32) -> Self {
        Self { delta_ang, ..self }
    }

    pub fn stepsize(self, stepsize: f32) -> Self {
        Self { stepsize, ..self }
    }

    pub fn draw_forward(self, draw_forward: &'a str) -> Self {
        Self {
            draw_forward,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn draw_backward(self, draw_backward: &'b str) -> Self {
        Self {
            draw_backward,
            ..self
        }
    }

    #[allow(dead_code)]
    pub fn forward(self, forward: &'c str) -> Self {
        Self { forward, ..self }
    }

    #[allow(dead_code)]
    pub fn backwards(self, backwards: &'d str) -> Self {
        Self { backwards, ..self }
    }

    fn classify(&self, symbol: char) -> Option<Step> {
        use Step::*;
        if self.draw_forward.contains(symbol) {
            Some(DrawForward)
        } else if self.draw_backward.contains(symbol) {
            Some(DrawBackward)
        } else if self.forward.contains(symbol) {
            Some(Forward)
        } else if self.backwards.contains(symbol) {
            Some(Backward)
        } else {
            None
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e> Turtle<'a, 'b, 'c, 'd, 'e> {
    pub fn with_config(config: &'a TurtleConfig<'b, 'c, 'd, 'e>) -> Self {
        Turtle {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            config,
        }
    }

    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn step_forward(&mut self) {
        self.x += f32::cos(self.angle) * self.config.stepsize;
        self.y += f32::sin(self.angle) * self.config.stepsize;
    }

    fn step_backwards(&mut self) {
        self.x -= f32::cos(self.angle) * self.config.stepsize;
        self.y -= f32::sin(self.angle) * self.config.stepsize;
    }

    fn turn_left(&mut self) {
        self.angle = (self.angle - self.config.delta_ang).rem_euclid(2.0 * std::f32::consts::PI);
    }

    fn turn_right(&mut self) {
        self.angle = (self.angle + self.config.delta_ang).rem_euclid(2.0 * std::f32::consts::PI);
    }

    pub fn draw<G, R>(mut self, graphics: &mut G, instructions: Instructions) -> Result<(), R>
    where
        G: Graphics<R>,
    {
        for instruction in instructions {
            use Instruction::*;
            match instruction {
                Symbol('+') => self.turn_left(),
                Symbol('-') => self.turn_right(),
                Symbol(c) => {
                    if let Some(step) = self.config.classify(c) {
                        let before = self.pos();
                        match step {
                            Step::Forward => {
                                self.step_forward();
                            }
                            Step::DrawForward => {
                                self.step_forward();

                                graphics.draw_line(before, self.pos())?;
                            }
                            Step::Backward => {
                                self.step_backwards();
                            }
                            Step::DrawBackward => {
                                self.step_backwards();
                                graphics.draw_line(before, self.pos())?;
                            }
                        }
                    }
                }
                Branch(ins) => self.clone().draw(graphics, ins)?,
            }
        }

        Ok(())
    }
}
