use crate::guard::Guard;
use crate::transition::{EmptyTransition, Transition, TransitionError};
use crate::Vertex;
use std::any::{Any, TypeId};

pub struct Choice<F, Tr1, Tr2> {
    choice: F,
    transition1: Tr1,
    transition2: Tr2,
}

impl<F> Choice<F, EmptyTransition, EmptyTransition> {
    pub fn if_<Event>(choice: F) -> Self
    where
        F: Guard<Event>,
    {
        Self {
            choice,
            transition1: EmptyTransition,
            transition2: EmptyTransition,
        }
    }

    pub fn then<Tr1, Event>(self, tr: Tr1) -> Choice<F, Tr1, EmptyTransition>
    where
        Tr1: Transition<Event>,
    {
        let Self { choice, .. } = self;
        Choice {
            choice,
            transition1: tr,
            transition2: EmptyTransition,
        }
    }
}

impl<F, Tr1> Choice<F, Tr1, EmptyTransition> {
    pub fn else_<Tr2, Event, Answer>(self, tr: Tr2) -> Choice<F, Tr1, Tr2>
    where
        Tr1: Transition<Event, Answer = Answer>,
        Tr2: Transition<Event, Answer = Answer>,
    {
        debug_assert_eq!(
            self.transition1.input_tid(),
            tr.input_tid(),
            "Inputs must be same for both transitions!"
        );

        let Self {
            choice,
            transition1,
            ..
        } = self;
        Choice {
            choice,
            transition1,
            transition2: tr,
        }
    }
}

impl<F, Tr1, Tr2, Event, Answer> Transition<Event> for Choice<F, Tr1, Tr2>
where
    F: Guard<Event>,
    Tr1: Transition<Event, Answer = Answer>,
    Tr2: Transition<Event, Answer = Answer>,
{
    type Answer = Answer;

    fn transition(
        &self,
        from: &mut dyn Vertex,
        event: Event,
    ) -> Result<(Box<dyn Any>, Self::Answer), TransitionError<Event>> {
        if self.choice.check(&event) {
            self.transition1.transition(from, event)
        } else {
            self.transition2.transition(from, event)
        }
    }

    fn input_tid(&self) -> TypeId {
        self.transition1.input_tid()
    }

    fn output_tids(&self) -> Vec<TypeId> {
        let mut tids = self.transition1.output_tids();
        tids.extend(self.transition2.output_tids());
        tids
    }
}
