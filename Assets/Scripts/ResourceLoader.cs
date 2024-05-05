using System;
using UnityEngine;

public class ResourceLoader : MonoBehaviour
{
    [Header( "Resources" )]
    public GameObject BattleUiPrefab;

    public GameObject OverworldUiPrefab;

    public GameObject StatScreenPrefab;

    [Header( "Listen to Event" )]
    [SerializeField] public VoidEventChannelSO m_LoadBattleUi;

    [SerializeField] public VoidEventChannelSO m_LoadUi;

    [SerializeField] private StringEventChannelSO m_LoadScene;
    
    private void Start()
    {
        DontDestroyOnLoad( gameObject );

        m_LoadUi.OnEventRaised       += OnLoadUi;
        m_LoadBattleUi.OnEventRaised += OnLoadBattleUi;
    }

    private void OnLoadBattleUi()
    {
        Instantiate( BattleUiPrefab );
    }

    private void OnLoadUi()
    {
        Instantiate( OverworldUiPrefab );
    }
}