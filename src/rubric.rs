use colored::*;

pub struct Category {
    pub id: u8,
    pub name: &'static str,
    pub question: &'static str,
    pub example: &'static str,
}

pub const CATEGORIES: &[Category] = &[
    Category { id: 1,  name: "Timing",                 question: "Does the timing feel suspicious or coincidental with other events?",        example: "A story about water contamination surfaces during a corporate scandal." },
    Category { id: 2,  name: "Emotional Manipulation",  question: "Does it provoke fear, outrage, or guilt without solid evidence?",             example: "Reports show crying children and dying wildlife but avoid causes." },
    Category { id: 3,  name: "Uniform Messaging",       question: "Are key phrases or ideas repeated across media?",                           example: "All outlets use terms like 'unprecedented' and 'avoidable tragedy'." },
    Category { id: 4,  name: "Missing Information",     question: "Are alternative views or critical details excluded?",                        example: "Few sources discuss the timeline or other possible contributors." },
    Category { id: 5,  name: "Simplistic Narratives",   question: "Is the story reduced to 'good vs. evil' frameworks?",                       example: "Blames one company entirely while ignoring systemic issues." },
    Category { id: 6,  name: "Tribal Division",         question: "Does it create an 'us vs. them' dynamic?",                                  example: "Locals are victims, while outsiders are blamed." },
    Category { id: 7,  name: "Authority Overload",      question: "Are questionable 'experts' driving the narrative?",                         example: "Non-environmental experts dominate airtime to support policies." },
    Category { id: 8,  name: "Call for Urgent Action",  question: "Does it demand immediate decisions without reflection?",                     example: "Campaigns push for immediate donations and rapid policy changes." },
    Category { id: 9,  name: "Overuse of Novelty",      question: "Is the event framed as shocking or unprecedented?",                         example: "Media emphasizes how 'shocking' and 'once-in-a-lifetime' the crisis is." },
    Category { id: 10, name: "Financial/Political Gain", question: "Do powerful groups benefit disproportionately?",                            example: "A company offering cleanup services lobbies for government contracts." },
    Category { id: 11, name: "Suppression of Dissent",  question: "Are critics silenced or labeled negatively?",                               example: "Opponents dismissed as 'deniers' or ignored." },
    Category { id: 12, name: "False Dilemmas",           question: "Are only two extreme options presented?",                                    example: "'Either you support this policy, or you don't care about the environment.'" },
    Category { id: 13, name: "Bandwagon Effect",         question: "Is there pressure to conform because 'everyone is doing it'?",              example: "Social media influencers post identical hashtags, urging followers to join." },
    Category { id: 14, name: "Emotional Repetition",     question: "Are the same emotional triggers repeated excessively?",                    example: "Destruction and suffering imagery looped endlessly on TV and online." },
    Category { id: 15, name: "Cherry-Picked Data",       question: "Are statistics presented selectively or out of context?",                   example: "Dramatic figures shared without explaining how they were calculated." },
    Category { id: 16, name: "Logical Fallacies",        question: "Are flawed arguments used to dismiss critics?",                             example: "Critics labeled 'out-of-touch elites' without addressing their points." },
    Category { id: 17, name: "Manufactured Outrage",     question: "Does outrage seem sudden or disconnected from facts?",                     example: "Viral memes escalate anger quickly with little context provided." },
    Category { id: 18, name: "Framing Techniques",       question: "Is the story shaped to control how you perceive it?",                      example: "The crisis is framed as entirely preventable, ignoring systemic factors." },
    Category { id: 19, name: "Rapid Behavior Shifts",    question: "Are groups adopting symbols or actions without clear reasoning?",           example: "Social media suddenly fills with users adding water droplet emojis to profiles." },
    Category { id: 20, name: "Historical Parallels",     question: "Does the story mirror manipulative past events?",                          example: "Past crises were similarly used to pass sweeping, controversial legislation." },
];

pub fn print_rubric() {
    println!("{}", "\n═══════════════════════════════════════════════════════".cyan());
    println!("{}", " NCI ENGINEERED REALITY SCORING SYSTEM — FULL RUBRIC".cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════\n".cyan());
    for cat in CATEGORIES {
        println!("{} {}", format!("[{:>2}]", cat.id).yellow().bold(), cat.name.bold());
        println!("     Q: {}", cat.question);
        println!("     E: {}\n", cat.example.dimmed());
    }
    println!("{}", "Scoring: 1 = Not Present  →  5 = Overwhelmingly Present".green());
    println!("{}", "─────────────────────────────────────────────────────────");
    println!("  0–25   {}", "Low likelihood of a PSYOP".green());
    println!("  26–50  {}", "Moderate likelihood — look deeper".yellow());
    println!("  51–75  {}", "Strong likelihood — manipulation likely".truecolor(255, 165, 0));
    println!("  76–100 {}\n", "Overwhelming signs of a PSYOP".red().bold());
}
