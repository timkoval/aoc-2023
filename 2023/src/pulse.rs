use itertools::{Either, Itertools};
use std::{collections::VecDeque, debug_assert};

use common::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    On,
    Off,
}

#[derive(Debug, Clone)]
enum Module<T> {
    Broadcast,
    FlipFlop(T, Mode),
    Conjunction(T, Vec<Option<Pulse>>),
    Output(T),
}

type Names<'a> = Vec<Option<&'a str>>;
type Modules<T> = Vec<(Module<T>, Vec<T>)>;
type FinalPeriods<T> = Vec<(usize, Option<T>)>;

pub fn solve(part: Part) -> Result<u64> {
    let (names, mut modules) = parse()?;
    let broadcaster_idx = names
        .iter()
        .positions(Option::is_none)
        .exactly_one()
        .map_err(|it| format_err!("wrong names: {:?}", it.count()))?;
    let mut params = match part {
        Part1 => Either::Left([0, 0]),
        Part2 => Either::Right(final_conjunctions(&names, &modules)?),
    };

    let mut queue = VecDeque::new();
    for step in 1..=part.value(1000, u16::MAX) {
        debug_assert!(queue.is_empty());
        queue.push_back((Pulse::Low, broadcaster_idx, broadcaster_idx));
        while let Some((pulse, src_idx, idx)) = queue.pop_front() {
            match (pulse, &mut params) {
                (Pulse::Low, Either::Left([low, _])) => *low += 1,
                (Pulse::High, Either::Left([_, high])) => *high += 1,
                (Pulse::Low, Either::Right((_, periods))) => {
                    for (ref i, period) in periods.iter_mut() {
                        if *i == idx {
                            if period.is_none() {
                                *period = Some(step);
                            }
                            break;
                        }
                    }
                    if let Some(res) = periods.iter().map(|(_, p)| p.map(u64::from)).product() {
                        #[cfg(debug_assertions)]
                        println!("Periods: {periods:?}");
                        return Ok(res);
                    }
                }
                (Pulse::High, Either::Right(_)) => {}
            }
            let (module, ref dsts) = &mut modules[idx];
            let new_pulse = match module {
                Module::Broadcast => {
                    debug_assert!(pulse == Pulse::Low);
                    pulse
                }
                Module::FlipFlop(_, mode) => {
                    if pulse == Pulse::High {
                        continue;
                    }
                    match mode {
                        Mode::On => {
                            *mode = Mode::Off;
                            Pulse::Low
                        }
                        Mode::Off => {
                            *mode = Mode::On;
                            Pulse::High
                        }
                    }
                }
                Module::Conjunction(_, input_pulses) => {
                    ensure!(input_pulses[src_idx].is_some(), "{}", src_idx);
                    input_pulses[src_idx] = Some(pulse);
                    if input_pulses.iter().all(|p| p != &Some(Pulse::Low)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    }
                }
                Module::Output(ref i) => {
                    if pulse == Pulse::Low && matches!(params, Either::Right((rx, _)) if rx == *i) {
                        return Ok(step.into());
                    }
                    continue;
                }
            };
            for &dst in dsts {
                queue.push_back((new_pulse, idx, dst));
            }
        }
    }

    match params {
        Either::Left([count_low_pulses, count_high_pulses]) => {
            #[cfg(debug_assertions)]
            println!("{count_low_pulses} low * {count_high_pulses} high");
            Ok(count_low_pulses * count_high_pulses)
        }
        Either::Right((_, periods)) => bail!("unprocessed periods: {:?}", periods),
    }
}

fn parse<'a>() -> Result<(Names<'a>, Modules<usize>)> {
    let input = include_str!("../inputs/pulse.txt");
    let mut modules: Modules<&str> = input
        .lines()
        .map(|line| {
            let (src, dsts) = line.split_once(" -> ").context("no ->")?;
            let src = if src == "broadcaster" {
                Module::Broadcast
            } else if let Some(s) = src.strip_prefix('%') {
                Module::FlipFlop(s, Mode::Off)
            } else if let Some(s) = src.strip_prefix('&') {
                Module::Conjunction(s, vec![])
            } else {
                bail!("wrong module: {}", src);
            };
            let dsts = dsts.split(", ").collect();
            Ok((src, dsts))
        })
        .try_collect()?;

    let mut names = modules
        .iter()
        .map(|(module, _)| match module {
            Module::Broadcast => None,
            Module::FlipFlop(s, _) | Module::Conjunction(s, _) => Some(*s),
            Module::Output(_) => unreachable!(),
        })
        .collect_vec();
    let other_names = modules
        .iter()
        .flat_map(|(_, dsts)| dsts)
        .filter(|s| !names.contains(&Some(**s)))
        .copied()
        .sorted()
        .dedup()
        .collect_vec();
    for &new_name in &other_names {
        names.push(Some(new_name));
        modules.push((Module::Output(new_name), vec![]));
    }
    let modules: Modules<usize> = modules
        .iter()
        .enumerate()
        .map(|(idx, (module, dsts))| {
            let module = match module {
                Module::Broadcast => Module::Broadcast,
                Module::FlipFlop(_, mode) => Module::FlipFlop(idx, *mode),
                Module::Conjunction(_, _) => Module::Conjunction(idx, vec![]),
                Module::Output(_) => Module::Output(idx),
            };
            dsts.iter()
                .map(|dst| {
                    names
                        .iter()
                        .position(|d| d.as_ref() == Some(dst))
                        .with_context(|| format_err!("unknown dst: {}", dst))
                })
                .try_collect()
                .map(|dsts| (module, dsts))
        })
        .try_collect()?;
    let modules = modules
        .iter()
        .map(|(module, dsts)| {
            let module = if let Module::Conjunction(idx, _) = *module {
                let input_pulses = modules
                    .iter()
                    .map(|(_, inputs)| inputs.contains(&idx).then_some(Pulse::Low))
                    .collect();
                Module::Conjunction(idx, input_pulses)
            } else {
                module.clone()
            };
            (module, dsts.clone())
        })
        .collect();
    Ok((names, modules))
}

fn final_conjunctions<T>(
    names: &Names,
    modules: &Modules<usize>,
) -> Result<(usize, FinalPeriods<T>)> {
    let rx_idx = names
        .iter()
        .position(|opt_name| opt_name == &Some("rx"))
        .context("no rx")?;
    let before_rx = modules
        .iter()
        .filter_map(|(module, dsts)| dsts.contains(&rx_idx).then_some(module))
        .collect_vec();
    ensure!(before_rx.len() == 1);
    let conjunction_before_rx_idx = match before_rx[..] {
        [Module::Conjunction(idx, _)] => *idx,
        _ => bail!("wrong before_rx"),
    };
    let periods = modules
        .iter()
        .filter(|(_, dsts)| dsts.contains(&conjunction_before_rx_idx))
        .map(|(module, _)| match module {
            Module::Conjunction(i, _) => Ok(*i),
            _ => bail!("wrong module"),
        })
        .map_ok(|i| (i, None))
        .try_collect()?;
    Ok((rx_idx, periods))
}
