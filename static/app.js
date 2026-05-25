// Protection discrète contre l'inspection simple
document.addEventListener('contextmenu', event => event.preventDefault());
document.onkeydown = function(e) {
    if (e.keyCode == 123) return false; // F12
    if (e.ctrlKey && e.shiftKey && e.keyCode == 'I'.charCodeAt(0)) return false; // Ctrl+Shift+I
    if (e.ctrlKey && e.shiftKey && e.keyCode == 'C'.charCodeAt(0)) return false; // Ctrl+Shift+C
    if (e.ctrlKey && e.shiftKey && e.keyCode == 'J'.charCodeAt(0)) return false; // Ctrl+Shift+J
    if (e.ctrlKey && e.keyCode == 'U'.charCodeAt(0)) return false; // Ctrl+U
};

let staticFishData = {};
let flatFishList = [];
let staticJunkData = {};
let flatJunkList = [];
let currentMuseumTab = 'fish'; // 'fish' or 'junk'
let allCatches = [];
let museumDiscoveries = [];
let currentRarity = 'all';
let currentType = 'all';
let currentPage = 1;
const itemsPerPage = 36;

const stateRanks = {
    'badly damaged': 1,
    'damaged': 2,
    'worn': 3,
    'good': 4,
    'pristine': 5
};

// Charger les données au démarrage
fetchLeaderboard();
fetchBananaKings();

loadFishData().then(() => {
    const path = window.location.pathname;
    if (path.startsWith('/player/')) {
        const username = path.split('/').pop();
        document.getElementById('usernameInput').value = username;
        fetchStats(username);
    }
});

async function loadFishData() {
    try {
        const response = await fetch('/api/fish_data');
        staticFishData = await response.json();
        flatFishList = [];
        for (const rarity in staticFishData) {
            staticFishData[rarity].forEach(fish => {
                flatFishList.push({
                    ...fish,
                    rarity: rarity
                });
            });
        }
        flatFishList.sort((a, b) => (a.id || 0) - (b.id || 0));

        // Fetch Junk Data
        const junkResponse = await fetch('/api/junk_data');
        staticJunkData = await junkResponse.json();
        flatJunkList = [];
        for (const rarity in staticJunkData) {
            staticJunkData[rarity].forEach(junk => {
                flatJunkList.push({
                    ...junk,
                    rarity: rarity
                });
            });
        }
        flatJunkList.sort((a, b) => (a.id || 0) - (b.id || 0));
        
        // Si le panneau du musée est déjà affiché (ex: fetchStats s'est terminé en premier), on le re-render
        if (document.getElementById('museumPane').style.display === 'block') {
            renderMuseum();
        }
    } catch (err) {
        console.error("Failed to load catalog data:", err);
    }
}

async function fetchLeaderboard() {
    try {
        const response = await fetch('/api/leaderboard');
        const data = await response.json();
        const list = document.getElementById('leaderboardList');
        list.innerHTML = '';
        data.top.forEach((p, i) => {
            const item = document.createElement('div');
            const rankClass = i === 0 ? 'rank-1' : i === 1 ? 'rank-2' : i === 2 ? 'rank-3' : '';
            item.className = `leaderboard-item ${rankClass}`.trim();
            item.style.cursor = 'pointer';
            item.onclick = (e) => {
                // If they clicked on the name link specifically, let the default anchor link navigation handle it
                if (e.target.tagName !== 'A') {
                    window.location.href = `/player/${p.username}`;
                }
            };
            
            // Header Row: Rank, Username, Level & Total Fish
            const headerRow = document.createElement('div');
            headerRow.className = 'leaderboard-header-row';
            
            const rankUser = document.createElement('div');
            rankUser.className = 'leaderboard-rank-user';
            
            const rankSpan = document.createElement('span');
            rankSpan.className = `leaderboard-rank ${rankClass}`.trim();
            rankSpan.textContent = `#${i+1}`;
            
            const nameLink = document.createElement('a');
            nameLink.className = 'leaderboard-username';
            nameLink.textContent = p.username;
            nameLink.href = `/player/${p.username}`;
            
            const levelSpan = document.createElement('span');
            levelSpan.className = 'leaderboard-level';
            levelSpan.textContent = `Niv. ${p.level}`;
            
            rankUser.appendChild(rankSpan);
            rankUser.appendChild(nameLink);
            rankUser.appendChild(levelSpan);
            
            const primarySummary = document.createElement('div');
            primarySummary.className = 'leaderboard-primary-summary';
            
            const pureFish = p.success - p.junk - p.banana - p.postcard - (p.gem || 0);
            primarySummary.innerHTML = `<strong>${pureFish}</strong> 🐟`;
            
            headerRow.appendChild(rankUser);
            headerRow.appendChild(primarySummary);
            
            // Stats Row: Pill badges for sub-stats
            const statsRow = document.createElement('div');
            statsRow.className = 'leaderboard-stats';
            
            const statsConfig = [
                { val: p.gold || 0, icon: '🪙', title: 'Or', class: 'gold' },
                { val: p.junk, icon: '🗑️', title: 'Déchets', class: 'junk' },
                { val: p.banana, icon: '🍌', title: 'Bananes', class: 'banana' },
                { val: p.gem || 0, icon: '💎', title: 'Gemmes', class: 'gem' },
                { val: p.postcard, icon: '📜', title: 'Cartes Postales', class: 'postcard' }
            ];
            
            statsConfig.forEach(stat => {
                const badge = document.createElement('span');
                badge.className = `l-stat ${stat.class}`;
                badge.title = stat.title;
                badge.innerHTML = `<strong>${stat.val}</strong> ${stat.icon}`;
                statsRow.appendChild(badge);
            });
            
            item.appendChild(headerRow);
            item.appendChild(statsRow);
            list.appendChild(item);
        });
    } catch (err) { console.error(err); }
}

async function fetchBananaKings() {
    try {
        const response = await fetch('/api/banana_kings');
        const data = await response.json();
        const list = document.getElementById('bananaKingsList');
        list.innerHTML = '';
        
        if (!data.history || data.history.length === 0) {
            list.innerHTML = '<p style="font-size: 0.8rem; color: #adadb8; text-align: center; padding: 10px; margin: 0;">Aucun Roi Banane n\'a encore été sacré ! 🍌</p>';
            return;
        }
        
        data.history.forEach(king => {
            const item = document.createElement('div');
            item.style.display = 'flex';
            item.style.flexDirection = 'column';
            item.style.padding = '8px 12px';
            item.style.background = king.dethroned_at ? 'rgba(38, 38, 44, 0.4)' : 'rgba(255, 215, 0, 0.08)';
            item.style.border = king.dethroned_at ? '1px solid rgba(58, 58, 61, 0.4)' : '1px solid rgba(255, 215, 0, 0.5)';
            item.style.borderRadius = '8px';
            item.style.boxShadow = king.dethroned_at ? 'none' : '0 0 10px rgba(255, 215, 0, 0.15)';
            item.style.transition = 'all 0.3s ease';
            
            const header = document.createElement('div');
            header.style.display = 'flex';
            header.style.justifyContent = 'space-between';
            header.style.alignItems = 'center';
            
            const nameLink = document.createElement('a');
            nameLink.textContent = `👑 @${king.username}`;
            nameLink.href = `/player/${king.username}`;
            nameLink.style.color = king.dethroned_at ? '#bf94ff' : '#ffd700';
            nameLink.style.textDecoration = 'none';
            nameLink.style.fontWeight = 'bold';
            nameLink.style.cursor = 'pointer';
            
            const badge = document.createElement('span');
            badge.style.fontSize = '0.7rem';
            badge.style.padding = '2px 6px';
            badge.style.borderRadius = '4px';
            badge.style.fontWeight = 'bold';
            if (king.dethroned_at) {
                badge.textContent = 'DÉTHRONÉ';
                badge.style.background = 'rgba(255, 79, 79, 0.1)';
                badge.style.color = '#ff4f4f';
            } else {
                badge.textContent = 'ACTUEL';
                badge.style.background = 'rgba(0, 230, 255, 0.15)';
                badge.style.color = '#00e6ff';
                badge.style.animation = 'shine 1.5s infinite alternate';
            }
            
            header.appendChild(nameLink);
            header.appendChild(badge);
            
            const details = document.createElement('div');
            details.style.fontSize = '0.75rem';
            details.style.color = '#adadb8';
            details.style.marginTop = '4px';
            details.style.display = 'flex';
            details.style.flexDirection = 'column';
            details.style.gap = '2px';
            
            const dateCrowned = new Date(king.crowned_at);
            const crownedStr = dateCrowned.toLocaleDateString('fr-FR', {
                day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit'
            });
            
            const crownedEl = document.createElement('span');
            crownedEl.innerHTML = `Sacré : <strong style="color: #efeff1;">${crownedStr}</strong>`;
            details.appendChild(crownedEl);
            
            if (king.dethroned_at) {
                const dateDethroned = new Date(king.dethroned_at);
                const dethronedStr = dateDethroned.toLocaleDateString('fr-FR', {
                    day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit'
                });
                
                const durationMs = dateDethroned - dateCrowned;
                const durationMins = Math.round(durationMs / 60000);
                let durationStr = `${durationMins} min`;
                if (durationMins >= 1440) {
                    durationStr = `${(durationMins / 1440).toFixed(1)} jours`;
                } else if (durationMins >= 60) {
                    durationStr = `${(durationMins / 60).toFixed(1)} h`;
                }
                
                const dethronedEl = document.createElement('span');
                dethronedEl.innerHTML = `Détrôné : <strong style="color: #efeff1;">${dethronedStr}</strong> <span style="color: #8f8f98; font-size: 0.7rem; margin-left: 5px;">(Règne : ${durationStr})</span>`;
                details.appendChild(dethronedEl);
            } else {
                const activeEl = document.createElement('span');
                activeEl.innerHTML = `<span style="color: #00ff00; font-weight: bold;">Règne en cours... 🌟</span>`;
                details.appendChild(activeEl);
            }
            
            item.appendChild(header);
            item.appendChild(details);
            list.appendChild(item);
        });
    } catch (err) {
        console.error(err);
        document.getElementById('bananaKingsList').innerHTML = '<p style="font-size: 0.8rem; color: #ff4f4f; text-align: center; padding: 10px; margin: 0;">Erreur lors du chargement de l\'historique.</p>';
    }
}

async function fetchStats(u) {
    const username = u || document.getElementById('usernameInput').value;
    if (!username) return;

    try {
        const response = await fetch(`/api/stats/${username}`);
        const data = await response.json();

        if (data.error) {
            document.getElementById('error').style.display = 'block';
            document.getElementById('statsDisplay').style.display = 'none';
            document.getElementById('museumPane').style.display = 'none';
            document.getElementById('commandGuideBox').style.display = 'block';
            document.getElementById('bananaKingsBox').style.display = 'block';
            document.getElementById('leaderboardBox').style.display = 'block';
        } else {
            document.getElementById('error').style.display = 'none';
            document.getElementById('statsDisplay').style.display = 'block';
            document.getElementById('museumPane').style.display = 'block';
            document.getElementById('commandGuideBox').style.display = 'none';
            document.getElementById('bananaKingsBox').style.display = 'none';
            document.getElementById('leaderboardBox').style.display = 'none';
            document.getElementById('res-username').innerText = data.username;
            document.getElementById('res-level').innerText = data.level;
            document.getElementById('res-vip').style.display = data.is_vip ? 'inline' : 'none';
            document.getElementById('res-xp-text').innerText = `${data.xp} / ${data.xp_next}`;
            const percent = (data.xp / data.xp_next) * 100;
            document.getElementById('res-xp-fill').style.width = `${percent}%`;
            
            document.getElementById('res-total').textContent = data.total;
            document.getElementById('res-success').textContent = data.success;
            document.getElementById('res-failed').textContent = data.failed;
            document.getElementById('res-stat-gold').textContent = data.gold || 0;

            const fishCount = data.success - data.junk - data.banana - data.postcard - (data.gem || 0);
            document.getElementById('res-stat-fish').textContent = fishCount;
            document.getElementById('res-stat-junk').textContent = data.junk;
            document.getElementById('res-stat-banana').textContent = data.banana;
            document.getElementById('res-stat-gem').textContent = data.gem || 0;
            document.getElementById('res-stat-postcard').textContent = data.postcard;

            // Remplir les statistiques de Coinflip
            const wins = data.coinflip_wins || 0;
            const losses = data.coinflip_losses || 0;
            const played = wins + losses;
            const cfBox = document.getElementById('coinflipStatsBox');
            if (played > 0) {
                cfBox.style.display = 'block';
                document.getElementById('web-cf-played').textContent = played;
                document.getElementById('web-cf-wins').textContent = wins;
                document.getElementById('web-cf-losses').textContent = losses;
                
                const netGold = (data.coinflip_gold_won_total || 0) - (data.coinflip_gold_lost_total || 0);
                const netText = netGold >= 0 ? `+${netGold} po` : `${netGold} po`;
                const netColor = netGold >= 0 ? '#00ff00' : '#ff4f4f';
                
                const netEl = document.getElementById('web-cf-net');
                netEl.textContent = netText;
                netEl.style.color = netColor;
                
                document.getElementById('web-cf-biggest-win').textContent = `+${data.coinflip_biggest_win || 0} po`;
                document.getElementById('web-cf-biggest-loss').textContent = `-${data.coinflip_biggest_loss || 0} po`;
            } else {
                cfBox.style.display = 'none';
            }

            // Stocker les captures
            allCatches = data.catches || [];
            museumDiscoveries = data.museum || [];

            // Affichage de la photo de profil Twitch
            const avatarUrl = data.profile_image_url;
            const avatarEl = document.getElementById('res-avatar');
            if (avatarUrl) {
                avatarEl.src = avatarUrl;
                avatarEl.style.display = 'block';
            } else {
                avatarEl.src = '';
                avatarEl.style.display = 'none';
            }

            // Logique "Roi des Bananes" & Couronne
            const hasBanana1 = allCatches.some(c => c.name === "Pristine Banana 1");
            const hasBanana2 = allCatches.some(c => c.name === "Pristine Banana 2");
            const isBananaKing = hasBanana1 && hasBanana2;

            document.getElementById('res-crown').style.display = isBananaKing ? 'block' : 'none';
            document.getElementById('res-banana-king-badge').style.display = isBananaKing ? 'inline' : 'none';

            // Divine Banana Showcase
            const bananaShowcase = document.getElementById('bananaShowcase');
            bananaShowcase.style.display = 'flex';

            const b1Slot = document.getElementById('banana1Slot');
            const b2Slot = document.getElementById('banana2Slot');

            if (hasBanana1) {
                b1Slot.classList.add('unlocked');
            } else {
                b1Slot.classList.remove('unlocked');
            }

            if (hasBanana2) {
                b2Slot.classList.add('unlocked');
            } else {
                b2Slot.classList.remove('unlocked');
            }

            if (hasBanana1 || hasBanana2) {
                bananaShowcase.classList.add('active');
            } else {
                bananaShowcase.classList.remove('active');
            }

            // Rendre les Trophées Éternels
            const trophiesSection = document.getElementById('trophiesSection');
            const trophiesContainer = document.getElementById('trophiesContainer');
            const trophies = data.trophies || [];
            
            if (trophies.length > 0) {
                trophiesSection.style.display = 'block';
                trophiesContainer.innerHTML = '';
                
                const emojiMap = {
                    'Bronze': '🥉',
                    'Argent': '🥈',
                    'Or': '🥇',
                    'Platinium': '💎',
                    'Diamant': '❄️',
                    'Obsidienne': '🌌',
                    'Night': '🌙',
                    'Voleur': '🍌',
                    'Eboueur': '🧹',
                    'Divin': '👑'
                };
                
                trophies.forEach(t => {
                    const badge = document.createElement('div');
                    const tierClass = t.trophy_tier.toLowerCase()
                        .replace('bronze', 'trophy-bronze')
                        .replace('argent', 'trophy-argent')
                        .replace('or', 'trophy-or')
                        .replace('platinium', 'trophy-platinium')
                        .replace('diamant', 'trophy-diamant')
                        .replace('obsidienne', 'trophy-obsidienne')
                        .replace('night', 'trophy-night')
                        .replace('voleur', 'trophy-voleur')
                        .replace('eboueur', 'trophy-eboueur')
                        .replace('divin', 'trophy-divin-special');
                        
                    badge.className = `trophy-badge ${tierClass}`;
                    
                    const emoji = emojiMap[t.trophy_tier] || '🏆';
                    
                    const dateUnlocked = new Date(t.unlocked_at);
                    const dateStr = dateUnlocked.toLocaleDateString('fr-FR', {
                        day: 'numeric', month: 'short', year: 'numeric'
                    });
                    
                    let displayName = t.trophy_tier;
                    if (t.trophy_tier === 'Night') displayName = 'Trophée de la Night';
                    if (t.trophy_tier === 'Voleur') displayName = 'Roi des Voleurs';
                    if (t.trophy_tier === 'Eboueur') displayName = 'Éboueur des Mers';
                    if (t.trophy_tier === 'Divin') displayName = 'Pêcheur Divin';
                    
                    badge.innerHTML = `
                        <span class="trophy-emoji">${emoji}</span>
                        <span class="trophy-season">${t.season.split(' ')[0]}</span>
                        <span class="trophy-tier-name">${displayName}</span>
                        <div class="trophy-tooltip">
                            <strong>🏆 ${displayName}</strong><br>
                            Saison : ${t.season}<br>
                            ${t.level > 0 ? `Niveau Final : ${t.level}<br>` : ''}
                            Débloqué le : ${dateStr}
                        </div>
                    `;
                    
                    trophiesContainer.appendChild(badge);
                });
            } else {
                trophiesSection.style.display = 'none';
            }

            // Rendre le Musée
            renderMuseum();

            // Réinitialiser la pagination et filtrer
            currentPage = 1;
            applyFilters();
        }
    } catch (err) {
        console.error(err);
    }
}

function filterByRarity(rarity) {
    currentPage = 1;
    currentRarity = rarity;
    const btns = document.querySelectorAll('#filterBar .filter-btn');
    btns.forEach(b => b.classList.toggle('active', b.getAttribute('data-rarity') === rarity));
    applyFilters();
}

function filterByType(type) {
    currentPage = 1;
    currentType = type;
    const btns = document.querySelectorAll('#typeFilterBar .filter-btn');
    btns.forEach(b => b.classList.toggle('active', b.getAttribute('data-type') === type));
    applyFilters();
}

// Réinitialiser la pagination lors de la saisie de recherche
document.getElementById('inventorySearch').addEventListener('input', () => {
    currentPage = 1;
});

function prevPage() {
    if (currentPage > 1) {
        currentPage--;
        applyFilters();
    }
}

function nextPage() {
    const searchTerm = document.getElementById('inventorySearch').value.toLowerCase();
    const filtered = allCatches.filter(f => {
        const isJunk = !!f.is_junk;
        const matchRarity = currentRarity === 'all' || f.rarity.toLowerCase() === currentRarity;
        const matchType = currentType === 'all' || 
                        (currentType === 'fish' && !isJunk) || 
                        (currentType === 'junk' && isJunk);
        const matchSearch = f.name.toLowerCase().includes(searchTerm);
        return matchRarity && matchType && matchSearch;
    });
    const totalPages = Math.max(1, Math.ceil(filtered.length / itemsPerPage));
    if (currentPage < totalPages) {
        currentPage++;
        applyFilters();
    }
}

function applyFilters() {
    const inv = document.getElementById('res-inventory');
    inv.innerHTML = '';
    
    const searchTerm = document.getElementById('inventorySearch').value.toLowerCase();

    const filtered = allCatches.filter(f => {
        const isJunk = !!f.is_junk;
        const matchRarity = currentRarity === 'all' || f.rarity.toLowerCase() === currentRarity;
        const matchType = currentType === 'all' || 
                        (currentType === 'fish' && !isJunk) || 
                        (currentType === 'junk' && isJunk);
        const matchSearch = f.name.toLowerCase().includes(searchTerm);
        return matchRarity && matchType && matchSearch;
    });

    // Logique de pagination
    const totalItems = filtered.length;
    const totalPages = Math.max(1, Math.ceil(totalItems / itemsPerPage));
    
    if (currentPage > totalPages) {
        currentPage = totalPages;
    }
    if (currentPage < 1) {
        currentPage = 1;
    }

    const startIndex = (currentPage - 1) * itemsPerPage;
    const endIndex = startIndex + itemsPerPage;
    const paginated = filtered.slice(startIndex, endIndex);

    const paginationControls = document.getElementById('paginationControls');
    if (totalItems > itemsPerPage) {
        paginationControls.style.display = 'flex';
        document.getElementById('paginationInfo').textContent = `Page ${currentPage} / ${totalPages}`;
        document.getElementById('btnPrevPage').disabled = (currentPage === 1);
        document.getElementById('btnNextPage').disabled = (currentPage === totalPages);
    } else {
        paginationControls.style.display = 'none';
    }

    if (paginated.length > 0) {
        paginated.forEach(f => {
            const slot = document.createElement('div');
            slot.className = 'fish-slot';
            if (f.is_junk) slot.classList.add('junk-item');
            
            const icon = document.createElement('span');
            icon.className = 'icon';
            
            const nameLower = f.name.toLowerCase();
            
            // Logique d'icônes étendue
            if (nameLower.includes('banana')) {
                icon.textContent = '🍌';
            } else if (nameLower.includes('gemme vip')) {
                icon.textContent = '💎';
            } else if (nameLower.includes('carte postale')) {
                icon.textContent = '📜';
            } else if (nameLower.includes('grenouille') || nameLower.includes('têtard')) {
                icon.textContent = '🐸';
            } else if (nameLower.includes('requin')) {
                icon.textContent = '🦈';
            } else if (nameLower.includes('anguille')) {
                icon.textContent = '🐍';
            } else if (nameLower.includes('calmar')) {
                icon.textContent = '🦑';
            } else if (nameLower.includes('piranha')) {
                icon.textContent = '🐡';
            } else if (nameLower.includes('thon') || nameLower.includes('saumon') || nameLower.includes('espadon')) {
                icon.textContent = '🍣';
            } else if (f.is_junk) {
                if (nameLower.includes('pneu')) icon.textContent = '🛞';
                else if (nameLower.includes('botte') || nameLower.includes('chaussure') || nameLower.includes('gant')) icon.textContent = '🥾';
                else if (nameLower.includes('canette') || nameLower.includes('bouteille') || nameLower.includes('gobelet') || nameLower.includes('soda')) icon.textContent = '🥫';
                else if (nameLower.includes('smartphone') || nameLower.includes('tablette') || nameLower.includes('téléphone') || nameLower.includes('clavier') || nameLower.includes('souris') || nameLower.includes('ordinateur') || nameLower.includes('pc')) icon.textContent = '💻';
                else if (nameLower.includes('pile') || nameLower.includes('batterie')) icon.textContent = '🔋';
                else if (nameLower.includes('radio') || nameLower.includes('téléviseur') || nameLower.includes('tv')) icon.textContent = '📻';
                else if (nameLower.includes('vélo') || nameLower.includes('moto') || nameLower.includes('scooter') || nameLower.includes('caddie')) icon.textContent = '🚲';
                else if (nameLower.includes('réfrigérateur') || nameLower.includes('lave-linge') || nameLower.includes('climatisation') || nameLower.includes('four')) icon.textContent = '🧊';
                else if (nameLower.includes('sac') || nameLower.includes('emballage') || nameLower.includes('plastique')) icon.textContent = '🛍️';
                else if (nameLower.includes('ancre') || nameLower.includes('moteur') || nameLower.includes('réacteur')) icon.textContent = '⚙️';
                else if (nameLower.includes('satellite') || nameLower.includes('station') || nameLower.includes('sonde') || nameLower.includes('capsule')) icon.textContent = '🛰️';
                else if (nameLower.includes('graal') || nameLower.includes('monolithe') || nameLower.includes('relique')) icon.textContent = '🏺';
                else icon.textContent = '🗑️';
            } else {
                icon.textContent = '🐟';
            }
            slot.appendChild(icon);

            const idLabel = document.createElement('div');
            idLabel.className = 'id-label';
            idLabel.textContent = `#${f.id || '?'}`;
            slot.appendChild(idLabel);

            const label = document.createElement('div');
            label.className = 'name-label';
            label.textContent = f.name;
            slot.appendChild(label);
            
            const rarityColors = {
                'common': '#efeff1',
                'uncommon': '#1fa363',
                'rare': '#00e6ff',
                'veryrare': '#9146ff',
                'epic': '#ff4ce2',
                'legendary': '#ffb444',
                'mythical': '#ff4f4f',
                'divin': '#ffffff'
            };
            const color = rarityColors[f.rarity.toLowerCase().replace(/\s/g, '')] || '#efeff1';
            slot.style.borderColor = color;
            if (f.rarity.toLowerCase() === 'divin') {
                slot.style.boxShadow = "0 0 10px #fff, inset 0 0 5px #fff";
            }

            const stateColors = {
                'badly damaged': '#ff4f4f',
                'damaged': '#ff8235',
                'worn': '#ffd700',
                'good': '#1fa363',
                'pristine': '#ffffff'
            };
            const stateColor = stateColors[f.state.toLowerCase()] || '#efeff1';

            const badge = document.createElement('div');
            badge.className = 'badge';
            if (f.state.toLowerCase() === 'pristine') {
                badge.classList.add('pristine-star');
                badge.textContent = '⭐';
            } else {
                badge.style.backgroundColor = stateColor;
            }
            slot.appendChild(badge);

            slot.onclick = () => showFishDetails(f, color);
            inv.appendChild(slot);
        });
    } else {
        inv.innerHTML = '<p style="font-size: 0.8rem; color: #adadb8; grid-column: span 4; text-align: center; padding: 20px;">Aucun objet trouvé.</p>';
    }
}

function formatWeight(g) {
    if (!g || isNaN(g) || g <= 0) return "-";
    if (g >= 1000000) return (g / 1000000).toFixed(2) + " T";
    if (g >= 1000) return (g / 1000).toFixed(2) + " kg";
    return g.toFixed(2) + " g";
}

// Recherche instantanée avec anti-rebond simple
let searchTimeout;
document.getElementById('inventorySearch').addEventListener('input', () => {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(applyFilters, 150);
});

function showFishDetails(fish, color) {
    let staticInfo = flatFishList.find(f => f.name.toLowerCase() === fish.name.toLowerCase());
    if (!staticInfo) {
        staticInfo = flatJunkList.find(f => f.name.toLowerCase() === fish.name.toLowerCase());
    }
    const idStr = (staticInfo && staticInfo.id) ? ` (#${String(staticInfo.id).padStart(3, '0')})` : '';
    document.getElementById('modal-name').textContent = fish.name + idStr;
    document.getElementById('modal-name').style.color = color;
    
    // Rétablir la mise en page normale du modal
    document.getElementById('modal-id-row').style.display = fish.is_junk ? 'none' : 'block';
    document.getElementById('modal-id').textContent = fish.id || '?';
    
    document.getElementById('modal-rarity').textContent = fish.rarity;
    document.getElementById('modal-rarity').style.color = color;
    
    const isJunk = fish.is_junk;
    document.getElementById('modal-size-label').textContent = "Taille:";
    document.getElementById('modal-weight-label').textContent = "Poids:";
    document.getElementById('modal-state-label').textContent = "État:";

    document.getElementById('modal-size-row').style.display = isJunk ? 'none' : 'block';
    document.getElementById('modal-weight-row').style.display = isJunk ? 'none' : 'block';
    document.getElementById('modal-state-row').style.display = isJunk ? 'none' : 'block';
    
    document.getElementById('modal-size').textContent = fish.size;
    document.getElementById('modal-weight').textContent = formatWeight(fish.weight);
    document.getElementById('modal-state').textContent = fish.state;
    document.getElementById('modal-desc').textContent = fish.description;

    // Chercher les métadonnées statiques pour enrichir
    const gpMeta = document.getElementById('modal-gameplay-meta');
    const ffContainer = document.getElementById('modal-fun-fact-container');
    
    if (staticInfo && !isJunk) {
        gpMeta.style.display = 'block';
        document.getElementById('modal-location').textContent = staticInfo.location || '-';
        document.getElementById('modal-price').textContent = staticInfo.price || '-';
        document.getElementById('modal-time').textContent = staticInfo.preferred_time || 'Toute la journée';
        document.getElementById('modal-season').textContent = staticInfo.preferred_season || "Toute l'année";
        
        if (staticInfo.fun_fact) {
            ffContainer.style.display = 'block';
            document.getElementById('modal-fun-fact').textContent = staticInfo.fun_fact;
        } else {
            ffContainer.style.display = 'none';
        }
    } else {
        gpMeta.style.display = 'none';
        ffContainer.style.display = 'none';
    }

    // Date et Stream
    const date = new Date(fish.caught_at);
    const dateStr = date.toLocaleDateString('fr-FR', {
        day: 'numeric', month: 'long', year: 'numeric', hour: '2-digit', minute: '2-digit'
    });
    const streamStr = fish.stream_title || "Hors live";
    const catcherStr = fish.caught_by ? `<br>🎣 Pêché par : <span style="color: #bf94ff; font-weight: bold;">@${fish.caught_by}</span>` : '';

    document.getElementById('modal-meta-row').innerHTML = `
        <p style="font-size: 0.8rem; color: #adadb8; margin-top: 10px;">
            📅 <span>${dateStr}</span><br>
            🎥 <span>${streamStr}</span>${catcherStr}
        </p>
    `;

    document.getElementById('fishModal').style.display = 'block';
    document.getElementById('modalOverlay').style.display = 'block';
    document.body.style.overflow = 'hidden';
}

function showMuseumDetails(pb) {
    let staticInfo = flatFishList.find(f => f.name.toLowerCase() === pb.name.toLowerCase());
    if (!staticInfo) {
        staticInfo = flatJunkList.find(f => f.name.toLowerCase() === pb.name.toLowerCase());
    }
    const idStr = (staticInfo && staticInfo.id) ? ` (#${String(staticInfo.id).padStart(3, '0')})` : '';
    const isUnlocked = pb.totalCaught > 0;

    document.getElementById('modal-name').textContent = (isUnlocked ? "Record : " : "Mystère : ") + pb.name + idStr;
    document.getElementById('modal-name').style.color = isUnlocked ? pb.color : '#8b8b9c';
    document.getElementById('modal-id-row').style.display = 'none';
    document.getElementById('modal-rarity').textContent = pb.rarity;
    document.getElementById('modal-rarity').style.color = pb.color;
    
    if (isUnlocked) {
        document.getElementById('modal-size-label').textContent = "Taille Record:";
        document.getElementById('modal-weight-label').textContent = "Poids Record:";
        document.getElementById('modal-state-label').textContent = "Meilleur État:";

        document.getElementById('modal-size-row').style.display = 'block';
        document.getElementById('modal-weight-row').style.display = 'block';
        document.getElementById('modal-state-row').style.display = 'block';
        
        document.getElementById('modal-size').textContent = pb.maxSize;
        document.getElementById('modal-weight').textContent = formatWeight(pb.maxWeight);
        document.getElementById('modal-state').textContent = pb.bestState;
    } else {
        document.getElementById('modal-size-row').style.display = 'none';
        document.getElementById('modal-weight-row').style.display = 'none';
        document.getElementById('modal-state-row').style.display = 'none';
    }
    
    document.getElementById('modal-desc').textContent = pb.description;

    // Chercher les métadonnées statiques pour enrichir
    const gpMeta = document.getElementById('modal-gameplay-meta');
    const ffContainer = document.getElementById('modal-fun-fact-container');
    
    if (staticInfo) {
        gpMeta.style.display = 'block';
        document.getElementById('modal-location').textContent = staticInfo.location || '-';
        document.getElementById('modal-price').textContent = staticInfo.price || '-';
        document.getElementById('modal-time').textContent = staticInfo.preferred_time || 'Toute la journée';
        document.getElementById('modal-season').textContent = staticInfo.preferred_season || "Toute l'année";
        
        if (staticInfo.fun_fact) {
            ffContainer.style.display = 'block';
            document.getElementById('modal-fun-fact').textContent = staticInfo.fun_fact;
        } else {
            ffContainer.style.display = 'none';
        }
    } else {
        gpMeta.style.display = 'none';
        ffContainer.style.display = 'none';
    }

    // Remplacer les métadonnées par les statistiques du Musée
    document.getElementById('modal-meta-row').innerHTML = `
        <p style="font-size: 0.8rem; color: #adadb8; margin-top: 10px;">
            📊 Total pêchés : <strong>${pb.totalCaught}</strong>
        </p>
    `;

    document.getElementById('fishModal').style.display = 'block';
    document.getElementById('modalOverlay').style.display = 'block';
    document.body.style.overflow = 'hidden';
}

function switchMuseumTab(tab) {
    currentMuseumTab = tab;
    
    // Toggle active class on buttons
    const btnFish = document.getElementById('btnMuseumFish');
    const btnJunk = document.getElementById('btnMuseumJunk');
    
    if (tab === 'fish') {
        btnFish.classList.add('active');
        btnJunk.classList.remove('active');
    } else {
        btnFish.classList.remove('active');
        btnJunk.classList.add('active');
    }
    
    renderMuseum();
}

function renderMuseum() {
    const grid = document.getElementById('museumGrid');
    grid.innerHTML = '';

    let discoveredCount = 0;
    const activeList = currentMuseumTab === 'fish' ? flatFishList : flatJunkList;
    const totalCount = activeList.length;

    activeList.forEach(item => {
        const discovery = museumDiscoveries.find(d => d.fish_name.toLowerCase() === item.name.toLowerCase());
        const isUnlocked = !!discovery;

        const slot = document.createElement('div');
        slot.className = 'museum-slot';

        // Badge ID style Pokédex
        const idStr = item.id ? `#${String(item.id).padStart(3, '0')}` : '#???';
        const idBadge = document.createElement('span');
        idBadge.className = 'id-badge';
        idBadge.textContent = idStr;
        slot.appendChild(idBadge);

        const icon = document.createElement('span');
        icon.className = 'icon';
        
        const nameLower = item.name.toLowerCase();
        
        if (currentMuseumTab === 'junk') {
            // Icon lookup for junk items
            if (nameLower.includes('botte') || nameLower.includes('chaussure')) {
                icon.textContent = '🥾';
            } else if (nameLower.includes('canette') || nameLower.includes('soda')) {
                icon.textContent = '🥫';
            } else if (nameLower.includes('pneu')) {
                icon.textContent = '⭕';
            } else if (nameLower.includes('plastique') || nameLower.includes('sac')) {
                icon.textContent = '🛍️';
            } else if (nameLower.includes('algue')) {
                icon.textContent = '🌿';
            } else if (nameLower.includes('trésor') || nameLower.includes('coffre')) {
                icon.textContent = '🏴‍☠️';
            } else {
                icon.textContent = '🗑️';
            }
        } else {
            // Icon lookup for fish items
            if (nameLower.includes('banana')) {
                icon.textContent = '🍌';
            } else if (nameLower.includes('grenouille') || nameLower.includes('têtard')) {
                icon.textContent = '🐸';
            } else if (nameLower.includes('requin')) {
                icon.textContent = '🦈';
            } else if (nameLower.includes('anguille')) {
                icon.textContent = '🐍';
            } else if (nameLower.includes('calmar')) {
                icon.textContent = '🦑';
            } else if (nameLower.includes('piranha')) {
                icon.textContent = '🐡';
            } else if (nameLower.includes('thon') || nameLower.includes('saumon') || nameLower.includes('espadon')) {
                icon.textContent = '🍣';
            } else {
                icon.textContent = '🐟';
            }
        }

        const nameLabel = document.createElement('div');
        nameLabel.className = 'name-label';
        nameLabel.textContent = item.name;

        const rarityColors = {
            'common': '#efeff1',
            'uncommon': '#1fa363',
            'rare': '#00e6ff',
            'veryrare': '#9146ff',
            'epic': '#ff4ce2',
            'legendary': '#ffb444',
            'mythical': '#ff4f4f',
            'divin': '#ffffff'
        };
        const color = rarityColors[item.rarity.toLowerCase().replace(/\s/g, '')] || '#efeff1';

        if (isUnlocked) {
            discoveredCount++;
            slot.classList.add('unlocked');
            slot.style.borderColor = color;
            if (item.rarity.toLowerCase() === 'divin') {
                slot.style.boxShadow = "0 0 8px #fff, inset 0 0 4px #fff";
            }

            const maxWeight = discovery.max_weight;
            const maxSize = discovery.max_size;
            const bestState = discovery.best_state;
            const bestDescription = discovery.description || item.fun_fact || "Un spécimen remarquable enregistré dans vos archives.";
            const totalCaught = discovery.total_caught;

            slot.appendChild(icon);
            slot.appendChild(nameLabel);

            // Badge d'état (comme dans l'inventaire)
            const stateColors = {
                'badly damaged': '#ff4f4f',
                'damaged': '#ff8235',
                'worn': '#ffd700',
                'good': '#1fa363',
                'pristine': '#ffffff'
            };
            const stateColor = stateColors[bestState.toLowerCase()] || '#efeff1';

            const badge = document.createElement('div');
            badge.className = 'badge';
            if (bestState.toLowerCase() === 'pristine') {
                badge.classList.add('pristine-star');
                badge.textContent = '⭐';
            } else {
                badge.style.backgroundColor = stateColor;
            }
            slot.appendChild(badge);

            // Compteur d'éléments pêchés en bas à gauche
            const countLabel = document.createElement('div');
            countLabel.className = 'catch-count';
            countLabel.textContent = `x${totalCaught}`;
            slot.appendChild(countLabel);

            slot.onclick = () => {
                showMuseumDetails({
                    name: item.name,
                    rarity: item.rarity,
                    color: color,
                    maxSize: maxSize,
                    maxWeight: maxWeight,
                    bestState: bestState,
                    description: bestDescription,
                    totalCaught: totalCaught
                });
            };
        } else {
            slot.classList.add('locked');
            
            const lockIcon = document.createElement('span');
            lockIcon.className = 'lock-icon';
            lockIcon.textContent = '🔒';
            slot.appendChild(lockIcon);

            icon.textContent = '❓';
            slot.appendChild(icon);
            slot.appendChild(nameLabel);

            slot.onclick = () => {
                showMuseumDetails({
                    name: item.name,
                    rarity: item.rarity,
                    color: color,
                    maxSize: null,
                    maxWeight: null,
                    bestState: null,
                    description: "Cette espèce mystérieuse n'a pas encore été enregistrée dans votre collection de la saison. Continuez à pêcher pour la découvrir !",
                    totalCaught: 0
                });
            };
        }

        grid.appendChild(slot);
    });

    // Mettre à jour la barre de progression globale
    document.getElementById('museum-discovered-count').textContent = discoveredCount;
    document.getElementById('museum-total-count').textContent = totalCount;
    
    const percentage = totalCount > 0 ? Math.round((discoveredCount / totalCount) * 100) : 0;
    document.getElementById('museum-percentage').textContent = `${percentage}%`;
    document.getElementById('museum-progress-fill').style.width = `${percentage}%`;
}

function closeModal() {
    document.getElementById('fishModal').style.display = 'none';
    document.getElementById('modalOverlay').style.display = 'none';
    document.body.style.overflow = '';
}

// Matrix Effect
const canvas = document.getElementById('matrix-bg');
const ctx = canvas.getContext('2d');

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789🐟🌊🎣⚓🦀🐡🦑🐙";
const fontSize = 16;
const columns = canvas.width / fontSize;
const drops = [];

for (let x = 0; x < columns; x++) {
    drops[x] = 1;
}

function drawMatrix() {
    ctx.fillStyle = "rgba(14, 14, 16, 0.05)";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = "#9146ff"; // Couleur violette Twitch
    ctx.font = fontSize + "px monospace";

    for (let i = 0; i < drops.length; i++) {
        const text = characters.charAt(Math.floor(Math.random() * characters.length));
        ctx.fillText(text, i * fontSize, drops[i] * fontSize);

        if (drops[i] * fontSize > canvas.height && Math.random() > 0.975) {
            drops[i] = 0;
        }
        drops[i]++;
    }
}

setInterval(drawMatrix, 33);

window.addEventListener('resize', () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
});
