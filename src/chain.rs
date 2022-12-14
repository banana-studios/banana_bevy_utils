use bevy::ecs::archetype::ArchetypeComponentId;
use bevy::ecs::component::ComponentId;
use bevy::ecs::query::Access;
use bevy::prelude::*;
use std::borrow::Cow;

pub struct ChainResultSystem<SystemIn, SystemOk, SystemErr> {
    system_in: SystemIn,
    system_ok: SystemOk,
    system_err: SystemErr,
    name: Cow<'static, str>,
    component_access: Access<ComponentId>,
    archetype_component_access: Access<ArchetypeComponentId>,
}

impl<
        T,
        E,
        O,
        SystemIn: System<Out = Result<T, E>>,
        SystemOk: System<In = T, Out = O>,
        SystemErr: System<In = E, Out = O>,
    > System for ChainResultSystem<SystemIn, SystemOk, SystemErr>
{
    type In = SystemIn::In;
    type Out = O;

    fn name(&self) -> Cow<'static, str> {
        self.name.clone()
    }

    fn archetype_component_access(&self) -> &Access<ArchetypeComponentId> {
        &self.archetype_component_access
    }

    fn component_access(&self) -> &Access<ComponentId> {
        &self.component_access
    }

    fn is_send(&self) -> bool {
        self.system_in.is_send() && self.system_ok.is_send() && self.system_err.is_send()
    }

    unsafe fn run_unsafe(&mut self, input: Self::In, world: &World) -> Self::Out {
        match self.system_in.run_unsafe(input, world) {
            Ok(t) => self.system_ok.run_unsafe(t, world),
            Err(e) => self.system_err.run_unsafe(e, world),
        }
    }

    fn apply_buffers(&mut self, world: &mut World) {
        self.system_in.apply_buffers(world);
        self.system_ok.apply_buffers(world);
        self.system_err.apply_buffers(world);
    }

    fn initialize(&mut self, world: &mut World) {
        self.system_in.initialize(world);
        self.system_ok.initialize(world);
        self.system_err.initialize(world);
        self.component_access
            .extend(self.system_in.component_access());
        self.component_access
            .extend(self.system_ok.component_access());
        self.component_access
            .extend(self.system_err.component_access());
    }

    fn update_archetype_component_access(&mut self, world: &World) {
        self.system_in.update_archetype_component_access(world);
        self.system_ok.update_archetype_component_access(world);
        self.system_err.update_archetype_component_access(world);

        self.archetype_component_access
            .extend(self.system_in.archetype_component_access());
        self.archetype_component_access
            .extend(self.system_ok.archetype_component_access());
        self.archetype_component_access
            .extend(self.system_err.archetype_component_access());
    }

    fn check_change_tick(&mut self, change_tick: u32) {
        self.system_in.check_change_tick(change_tick);
        self.system_ok.check_change_tick(change_tick);
        self.system_err.check_change_tick(change_tick);
    }

    fn is_exclusive(&self) -> bool {
        self.system_in.is_exclusive()
            && self.system_ok.is_exclusive()
            && self.system_err.is_exclusive()
    }

    fn get_last_change_tick(&self) -> u32 {
        self.system_in.get_last_change_tick()
    }

    fn set_last_change_tick(&mut self, last_change_tick: u32) {
        self.system_in.set_last_change_tick(last_change_tick);
        self.system_ok.set_last_change_tick(last_change_tick);
        self.system_err.set_last_change_tick(last_change_tick);
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Into Chain System
//////////////////////////////////////////////////////////////////////////////////////////

pub trait IntoChainResultSystem<T, E, Out, SysOk, SysErr, ParamIn, ParamOk, ParamErr>:
    IntoSystem<(), Result<T, E>, ParamIn> + Sized
where
    SysOk: IntoSystem<T, Out, ParamOk>,
    SysErr: IntoSystem<E, Out, ParamErr>,
{
    fn chain_result(
        self,
        ok: SysOk,
        err: SysErr,
    ) -> ChainResultSystem<Self::System, SysOk::System, SysErr::System>;
}

impl<T, E, Out, SysIn, SysOk, SysErr, ParamIn, ParamOk, ParamErr>
    IntoChainResultSystem<T, E, Out, SysOk, SysErr, ParamIn, ParamOk, ParamErr> for SysIn
where
    SysIn: IntoSystem<(), Result<T, E>, ParamIn>,
    SysOk: IntoSystem<T, Out, ParamOk>,
    SysErr: IntoSystem<E, Out, ParamErr>,
{
    fn chain_result(
        self,
        ok: SysOk,
        err: SysErr,
    ) -> ChainResultSystem<SysIn::System, SysOk::System, SysErr::System> {
        let system_in = IntoSystem::into_system(self);
        let system_ok = IntoSystem::into_system(ok);
        let system_err = IntoSystem::into_system(err);

        ChainResultSystem {
            name: Cow::Owned(format!(
                "ChainResult({} -> {} / {})",
                system_in.name(),
                system_ok.name(),
                system_err.name()
            )),
            system_in,
            system_ok,
            system_err,
            archetype_component_access: Default::default(),
            component_access: Default::default(),
        }
    }
}
